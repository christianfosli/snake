## Terraform / Infrastructure

Provisions infrastructure needed for our webapp / api.

**Note on database**
I would like to provision the database here to.
Unfortunately creating free tier (M0) MongoDB clusters is not supported
by MongoDB API/terraform provider, and therefore I create it manually on
mongodb.com instead.
