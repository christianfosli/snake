sleep 30s

echo "running db init script"
/opt/mssql-tools/bin/sqlcmd -s localhost -U sa -P $SA_PASSWORD -d master -i schema.sql
echo "db init script finished"
