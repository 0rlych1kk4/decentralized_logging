# decentralized_logging
A Proof of Concept (PoC) for decentralized logging using Rust. This project demonstrates how to set up a logging client and server to handle distributed log data collection and aggregation.

# Decentralized Logging
A Proof of Concept (PoC) for decentralized logging using Rust. This project demonstrates how to set up a logging client and server to handle distributed log data collection and aggregation.

## Features

- **Logging Client**:
  - Periodically generates log messages.
  - Sends log messages to the logging server using HTTP POST requests.
- **Logging Server**:
  - Receives log messages from clients.
  - Stores received log messages in memory.
  - Provides an HTTP GET endpoint to retrieve all stored logs.

## Getting Started

### Prerequisites

- Rust

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/0rlych1kk4/decentralized_logging.git
   cd decentralized_logging