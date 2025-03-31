# Solana Blockchain Data Aggregator

## Overview
This project is a lightweight yet powerful data aggregator designed to collect and process transfer transactions from the Solana blockchain. Instead of relying on inefficient polling, we take the smarter route—leveraging Solana’s WebSocket-based RPC API for real-time, optimized, and reliable data streaming.

## How It Works:
WebSocket Magic: We subscribe to logs, identify transfer transactions, and parse them on the fly.

Global Data Storage: Parsed transactions are stored in a global value for easy access.

RESTful API: An API layer neatly exposes this aggregated data, responding to various queries you mentioned.

## What’s Next?
Now, is this the perfect version? Not quite. There’s still room for improvement—better security, scalability, and other refinements. But given time constraints, this is the best I could craft for now, and I hope you’ll appreciate the effort!

Best,


---

## Architecture

The application consists of the following components:

1. **WebSocket Module**: Handles real-time updates from the Solana blockchain.
2. **Transaction Processor**: Processes and organizes transaction data.
3. **Data Store**: Maintains an in-memory structure for storing transaction and account data.
4. **API Module**: Provides a RESTful API for external systems to query the aggregated data.

---

## API Endpoints

### 1. `/transactions`
- **Description**: Retrieves transaction data based on query parameters.
- **Query Parameters**:
  - `id`: Fetches a specific transaction by its ID.
  - `day`: Fetches transactions for a specific day (format: `DD/MM/YYYY`).
- **Examples**:
  - `GET /transactions?id=4CqYTMNtGpWjk67Ntq9QtDHZNaDeqYwhbh6cMVx7Qx6Y4b43kgsHP8t4TJbdrWf5kD4xuWNXhFLZfo4H6GBmxXzG`
  - `GET /transactions?day=23/05/2023`

### 2. `/`
- **Description**: Health check endpoint to verify the server is running.
- **Example**:
  - `GET /`
