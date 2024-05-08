## Infrastructure as Code (IaC)
   https://www.coursera.org/learn/infrastructure-automation-with-terraform/lecture/bhRHs/introduction-to-the-course

## Need for IaC
   * Advantages: 
     Full traceability 
   * High Efficiency: less prone to errors.
   * Scalability. how would you build this or write the configuration for 1000 servers? 


## Alternatives to Terraform:
   * AWS Cloud Form 9
   * Ansible ()
   * Google Cloud Deployment Manager (only for Google)

   Terraform has the advantage of being declarative.
   You just need to define the outcome and intermediate steps shall be self-figured.

   * Accountability, High Efficiency without errors and differences. 

   * Infrastructure Automation with Terraform

## Terraform Comparison of IaC Tools
   
   Cloud Native and Cloud Agnostic Tools
   * Cloud providers have their own deployment automation technologies.


   I prefer cloud agnostic tools like Hashicorp and Ansible.

   Example are: terraform, Chef,Puppet and ansible 

## Comparison of IaC Tools: 
   * IaC (bllueprint of data center) --> versioned -->
   shared and reused. 
   * terraform: execution plans:  
   * resource graph
   * Files are treated as Code.


## Mutable vs Immutable architecture
   mutable approach : traditional: 
   Terraform immutable: servers never modified.
   
   Mutable vs immutable architectures. 

## Declarative vs Procedure
   Declarative: code to define end state.

## Master and Agents
   Terraform master less. communicate with cloud providers using the cloud providers APIs

   No need for agent software. 

   With others: Chef, Puppet, SaltStack you need to install agents for master-slave solution where agent perform the scaling processes.


## State Files:
    Terraform: state files for the state machines.
    all config files in a state file. Metadata and improve performance for large infrastructure.
    example: tf.state.

    When finally built: terraform.tfstate.
    
## State locking:
   when in process of change, tf.state is locked, once in finished, the lock is deactivated.
   
## Assignment to complete 
   (DONE)


## How terraform works?
   * written in Go.
   * binary to deploy infrastructure
   * no need for other tools
   * on the Backend: terraform makes API calls to all the calls to the different
     apis.

   * IBM bought hashicorp right now
   * Terraform makes RPC (remote procedure calls) to the plugins which contatins
     providers and provisioners.

## Responsibilities of terraform Core: 
   * Infrastructure as code
   * Resource state management
   * Construction of the Rosource Graph
   * Plan execution
   * communication with plugins

##  Terraform Plugins
    * plugin is a server.
    * Core invokes plugin over RPC.
    * CRUD (create,reade,update,delete)

## Terraform init
   * terraform init
   what does it do? why do we need it?

   init initialize the working directory containing Terraform configuration files.

   * read config files and determine which plugins are necessary.
   * search and decide which plugin version to use
   * writes lock file until process is complete.

## Terraform plan
   * terraform plan allows to preview the changes and ask for specifics if it were not provided before.

## Terraform destroy to destroy all the currently built architectures.
   terraform destroys all built resources such that you build it all again.

## Plugins
   * AWS, GCP, Microsoft Azure, OpenStack

   * Shared Credentials File: AWS CLI: 

   * For AWS we have EC2 Role with IAM role. 

## Providers (Individual):
   provider meta arguments: version and alias
   What does this alias do? It provides a way:

## Plugins: Providers

## Deploying Resources AWS - EC2 & S3.
   AWS Provider page.

##  