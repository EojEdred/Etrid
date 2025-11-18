#!/usr/bin/env node

const { Pool } = require('pg');
const fs = require('fs');
const path = require('path');
require('dotenv').config();

const pool = new Pool({
  host: process.env.DB_HOST || 'localhost',
  port: parseInt(process.env.DB_PORT || '5432'),
  database: process.env.DB_NAME || 'etrid_wallet',
  user: process.env.DB_USER || 'postgres',
  password: process.env.DB_PASSWORD,
});

async function migrate() {
  const client = await pool.connect();

  try {
    console.log('Starting database migration...');

    // Read schema file
    const schemaPath = path.join(__dirname, '..', 'src', 'database', 'schema.sql');
    const schema = fs.readFileSync(schemaPath, 'utf8');

    // Execute schema
    await client.query('BEGIN');
    await client.query(schema);
    await client.query('COMMIT');

    console.log('Migration completed successfully!');
  } catch (error) {
    await client.query('ROLLBACK');
    console.error('Migration failed:', error.message);
    process.exit(1);
  } finally {
    client.release();
    await pool.end();
  }
}

migrate();
