# bhu_db

A lightweight, SQLite-like database written in Rust with a focus on **ACID compliance** and core SQL functionalities. The primary goal of this project is to **learn the internals of database systems** by building one from scratch.

## 🚀 Project Overview


`bhu_db` is an experimental database that aims to support the following features:

- **ACID Compliance**: Ensuring transactions are Atomic, Consistent, Isolated, and Durable.
- **Core SQL Statements**:
  - `SELECT`: Query data from tables.
  - `INSERT`: Add new data to tables.
  - `UPDATE`: Modify existing data.
  - `DELETE`: Remove data from tables.
- **Network Connectivity**: Access the database over a network connection.
- **Data Types**: Support for primitive data types, including:
  - Integers
  - Floats
  - Strings
  - Booleans
- **Indexing**: Improve performance for data retrieval.
- **Persistence Store**: Store data on disk for durability.
- **Transactions**: Support for `BEGIN`, `COMMIT`, and `ROLLBACK`.

## 📚 Learning Objectives

The primary purpose of `bhu_db` is to **understand how databases work under the hood**. This project is a learning exercise and is not intended for production use. Key learning outcomes include:

- Parsing SQL statements.
- Implementing data storage and indexing.
- Handling concurrency and transaction management.
- Ensuring data durability and crash recovery.

## 🛠️ Features & Goals

- **ACID Compliance**: Guarantees for transactions to be atomic, consistent, isolated, and durable.
- **SQL Support**: 
  - Basic support for `SELECT`, `INSERT`, `UPDATE`, and `DELETE` statements.
  - Transaction commands: `BEGIN`, `COMMIT`, and `ROLLBACK`.
- **Data Types**: Support for commonly used data types:
  - Integer
  - Float
  - String
  - Boolean
- **Persistence**: Data is stored on disk using efficient I/O operations to ensure durability.
- **Indexing**: Basic indexing support using B-Trees for faster lookups.
- **Networking**: Ability to connect to the database over a TCP/IP connection.
- **REPL Interface**: A command-line interface to interact with the database.

> [!NOTE]  
> This is work in progress, as I am going throught some books to nail the theory, so might take a while for the implementation. 
