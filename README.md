# terac

> Tera is a template engine inspired by Jinja2 and the Django template language.

This program is a simple wrapper around the [Tera](https://github.com/Keats/tera) template engine. 

It allows you to render template files with values from files in your command line.

## Usage

```shell

$ docker run --rm -it ghcr.io/amphineko/terac terac --help

Usage: terac [OPTIONS] [TEMPLATE_FILE]

Arguments:
  [TEMPLATE_FILE]  Template file to render

Options:
  -a, --values <VALUE_FILES>
          Files containing values to be used in the template
  -i, --include <INCLUDED_TEMPLATE_FILES>
          Included templates to be rendered and used in the main template as `.includes`
  -o, --output <OUTPUT_FILE>
          Output file of the rendered template
  -h, --help
          Print help

```

## Example

### Multiple templates and value files

Build a Fedora CoreOS ignition file from multiple Jinja2 templates and value files

```console
$ (cat <<EOF
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
        # {% filter indent %}
        # {% include "hostname_file" %}
        # {% endfilter %}
EOF
) | docker run --rm -i -v ./playground:/playground ghcr.io/amphineko/terac terac \
    -a /playground/base.json \
    -a /playground/test.json \
    -i hostname_file=/playground/hostname.yml.j2
```

### Output

```yaml
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
            inline: "test-worker1"

```

## License

MIT

