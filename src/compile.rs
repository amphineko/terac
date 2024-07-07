use std::collections::HashMap;

use tera::Tera;

use crate::json::patch_json;

fn merge_values(values: &[serde_json::Value]) -> serde_json::Value {
    match values.first() {
        Some(first) => values
            .iter()
            .skip(1)
            .fold(first.clone(), |acc, value| patch_json(&acc, value)),
        None => serde_json::Value::Object(serde_json::Map::new()),
    }
}

pub fn compile(
    template: &str,
    included_templates: &HashMap<String, String>,
    all_values: &[serde_json::Value],
) -> Result<String, Box<dyn std::error::Error>> {
    let context = tera::Context::from_serialize(merge_values(all_values))
        .map_err(|err| format!("Cannot create context from values: {err}"))?;

    let mut tera = Tera::default();
    for (name, content) in included_templates {
        tera.add_raw_template(name, content)
            .map_err(|err| format!("Cannot load included template {name}: {err}"))?;
    }

    tera.add_raw_template("main", template)
        .map_err(|err| format!("Cannot load main template: {err}"))?;

    let output = tera
        .render("main", &context)
        .map_err(|err| format!("Cannot render template: {:#?}", err))?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_values() {
        let values = vec![
            serde_json::json!({"a": 1, "b": 1, "c": 1}),
            serde_json::json!({"b": 2, "c": null, "d": 2}),
        ];

        let expected = serde_json::json!({"a": 1, "b": 2, "d": 2});

        assert_eq!(merge_values(&values), expected);
    }

    #[test]
    fn test_compile() {
        let template = r#"
            variant: {{ node.host_variant }}
            version: "1.5.0"
            storage:
                disks:
                -   device: /dev/disk/by-id/coreos-boot-disk
                    wipe_table: false
                    partitions:
                    -   label: root
                        number: 4
                        size_mib: {{ common.root_size }}
                        resize: true
                    -   label: var
                        size_mib: 0

                filesystems:
                    -   path: /var
                        device: /dev/disk/by-partlabel/var
                        format: {{ common.var_fs }}
                        with_mount_unit: true

                files:
{% include "hostname_file" %}
        "#;

        let included_templates: HashMap<String, String> = HashMap::from_iter(vec![(
            "hostname_file".into(),
            vec![
"                    -   path: /etc/hostname",
"                        mode: 0644",
"                        contents:",
"                            inline: \"{{ common.cluster_prefix }}{{ node.name }}\"",
            ]
            .join("\n"),
        )]);

        let values = vec![
            serde_json::json!({
                "common": {
                    "cluster_prefix": "test-",
                    "root_size": 8192,
                    "var_fs": "btrfs",
                },
            }),
            serde_json::json!({
                "node": {
                    "host_variant": "fcos",
                    "name": "node"
                }
            }),
        ];

        let expected = r#"
            variant: fcos
            version: "1.5.0"
            storage:
                disks:
                -   device: /dev/disk/by-id/coreos-boot-disk
                    wipe_table: false
                    partitions:
                    -   label: root
                        number: 4
                        size_mib: 8192
                        resize: true
                    -   label: var
                        size_mib: 0

                filesystems:
                    -   path: /var
                        device: /dev/disk/by-partlabel/var
                        format: btrfs
                        with_mount_unit: true

                files:
                    -   path: /etc/hostname
                        mode: 0644
                        contents:
                            inline: "test-node"
        "#;

        println!("Expected:\n{}", expected);
        println!(
            "Actual:\n{}",
            compile(template, &included_templates, &values).unwrap()
        );

        assert_eq!(
            compile(template, &included_templates, &values).unwrap(),
            expected
        );
    }
}
