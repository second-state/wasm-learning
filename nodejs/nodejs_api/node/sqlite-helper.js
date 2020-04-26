const sqlite3 = require('better-sqlite3');

function create(path) {
  const db = new sqlite3(path);
  db.close();
}

function query(path, sql) {
  const db = new sqlite3(path);
  const stmt = db.prepare(sql);
  const rows = stmt.all();
  db.close();
  return JSON.stringify(rows);
}

function update(path, sql) {
  const db = new sqlite3(path);
  const stmt = db.prepare(sql);
  stmt.run();
  db.close();
}

function exec(path, sql) {
  const db = new sqlite3(path);
  db.exec(sql);
  db.close();
}

module.exports = { create, query, update, exec }
