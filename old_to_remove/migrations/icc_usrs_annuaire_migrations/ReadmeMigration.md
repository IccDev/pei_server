[Link](https://github.com/jaemk/migrant)

# Install with postgres

```cargo install migrant --features postgres```

# Initial the project

```migrant init```    - Initialize project by creating a Migrant.toml



# Verify config

```migrant setup```   - Verify database info/credentials and setup a __migrant_migrations table if missing.



# create new migration

```migrant new <tag>``` - Generate new up & down files with the given <tag>

# Apply migration

```migrant apply```