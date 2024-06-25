# Command-Line Help for `binarite-cli`

This document contains the help content for the `binarite-cli` command-line program.

**Command Overview:**

* [`binarite-cli`↴](#binarite-cli)
* [`binarite-cli login`↴](#binarite-cli-login)
* [`binarite-cli logout`↴](#binarite-cli-logout)
* [`binarite-cli sphere`↴](#binarite-cli-sphere)
* [`binarite-cli sphere create`↴](#binarite-cli-sphere-create)
* [`binarite-cli sphere delete`↴](#binarite-cli-sphere-delete)
* [`binarite-cli sphere list`↴](#binarite-cli-sphere-list)
* [`binarite-cli sphere status`↴](#binarite-cli-sphere-status)
* [`binarite-cli sphere output`↴](#binarite-cli-sphere-output)
* [`binarite-cli project`↴](#binarite-cli-project)

## `binarite-cli`

Metagravity's Platform cli

**Usage:** `binarite-cli <COMMAND>`

###### **Subcommands:**

* `login` — Login to Metagravity's platform using Auth0
* `logout` — Remove Metagravity's platform stored credential
* `sphere` — Create, Delete, List Platform Metaspheres
* `project` — Create, Delete, List Platform Projects (Unimplemented)



## `binarite-cli login`

Login to Metagravity's platform using Auth0

**Usage:** `binarite-cli login`



## `binarite-cli logout`

Remove Metagravity's platform stored credential

**Usage:** `binarite-cli logout`



## `binarite-cli sphere`

Create, Delete, List Platform Metaspheres

**Usage:** `binarite-cli sphere
       sphere <COMMAND>`

###### **Subcommands:**

* `create` — Create a new Metasphere
* `delete` — Delete a Metasphere using its id
* `list` — List all Metaspheres within a project
* `status` — Retrieve Metasphere status given its Id (Unimplemented)
* `output` — Retrieve Metasphere Output variables



## `binarite-cli sphere create`

Create a new Metasphere

**Usage:** `binarite-cli sphere create [OPTIONS] --project-id <PROJECT_ID> --name <NAME> --template <TEMPLATE>`

###### **Options:**

* `-p`, `--project-id <PROJECT_ID>` — Id of the project to create the metasphere in
* `-n`, `--name <NAME>` — Name of the metasphere to create
* `-t`, `--template <TEMPLATE>` — Template used to create the metasphere
* `--cloud-provider <CLOUD_PROVIDER>` — Cloud provider to use to create the metasphere
* `--cloud-region <CLOUD_REGION>` — Cloud region where to create the metasphere
* `--instance-count <INSTANCE_COUNT>` — How many replicas to create (template-dependant)
* `--instance-size <INSTANCE_SIZE>` — T-Shirt Size of the main VM(s) of the template. See https://app.clickup.com/9005002661/v/dc/8cbuvx5-59952 for mor info

  Possible values: `s`, `m`, `l`, `xl`

* `--custom-args <CUSTOM_ARGS>` — Custom arguments to pass to the template. Comma separated key=value pairs



## `binarite-cli sphere delete`

Delete a Metasphere using its id

**Usage:** `binarite-cli sphere delete --metasphere-id <METASPHERE_ID>`

###### **Options:**

* `-m`, `--metasphere-id <METASPHERE_ID>` — Id of the metasphere to delete



## `binarite-cli sphere list`

List all Metaspheres within a project

**Usage:** `binarite-cli sphere list [OPTIONS] --project-id <PROJECT_ID>`

###### **Options:**

* `-p`, `--project-id <PROJECT_ID>` — Id of the project for listing the metaspheres
* `-s`, `--show-deleted` — Also include deleted metaspheres in the listing



## `binarite-cli sphere status`

Retrieve Metasphere status given its Id (Unimplemented)

**Usage:** `binarite-cli sphere status`



## `binarite-cli sphere output`

Retrieve Metasphere Output variables

**Usage:** `binarite-cli sphere output [OPTIONS] --metasphere-id <METASPHERE_ID>`

###### **Options:**

* `-m`, `--metasphere-id <METASPHERE_ID>` — Id of the metasphere to show the output
* `--public-ip` — Filter output to only show public ip(s) of the metasphere



## `binarite-cli project`

Create, Delete, List Platform Projects (Unimplemented)

**Usage:** `binarite-cli project`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

