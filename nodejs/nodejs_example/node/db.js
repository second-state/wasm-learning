const { create_sqlite, query_sqlite } = require('../pkg/nodejs_example.js');
  
create_sqlite("test.sqlite");
query_sqlite("test.sqlite");
