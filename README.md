Dework Program
==============

The **Dework Program** is a decentralized work management platform built on the Solana blockchain using the Anchor framework. This smart contract enables seamless interactions between clients, workers, and arbitrators by facilitating job creation, quote submissions, quote acceptance, dispute resolution, and secure payment releases. Designed with a focus on security, efficiency, and practical decentralized operations, this project is engineered for production-grade deployments.

* * * * *

Table of Contents
-----------------

-   [Overview](https://chatgpt.com/c/67d6c65a-dea8-8002-97d3-33dc4c636f13#overview)
-   [Features](https://chatgpt.com/c/67d6c65a-dea8-8002-97d3-33dc4c636f13#features)
-   [Architecture](https://chatgpt.com/c/67d6c65a-dea8-8002-97d3-33dc4c636f13#architecture)
-   [Getting Started](https://chatgpt.com/c/67d6c65a-dea8-8002-97d3-33dc4c636f13#getting-started)
-   [Deployment](https://chatgpt.com/c/67d6c65a-dea8-8002-97d3-33dc4c636f13#deployment)
-   [Usage](https://chatgpt.com/c/67d6c65a-dea8-8002-97d3-33dc4c636f13#usage)
-   [Contract Structure](https://chatgpt.com/c/67d6c65a-dea8-8002-97d3-33dc4c636f13#contract-structure)
-   [Testing](https://chatgpt.com/c/67d6c65a-dea8-8002-97d3-33dc4c636f13#testing)
-   [Contributing](https://chatgpt.com/c/67d6c65a-dea8-8002-97d3-33dc4c636f13#contributing)
-   [License](https://chatgpt.com/c/67d6c65a-dea8-8002-97d3-33dc4c636f13#license)
-   [Contact](https://chatgpt.com/c/67d6c65a-dea8-8002-97d3-33dc4c636f13#contact)

* * * * *

Overview
--------

The Dework Program leverages the Anchor framework to implement a suite of functionalities critical for a decentralized work platform. This contract orchestrates interactions between multiple stakeholders---clients, workers, and arbitrators---by managing jobs, quotes, disputes, and escrowed payments.

Key functionalities include:

-   **Job Lifecycle Management:** Create, update, and delete jobs with built-in validations.
-   **Quote Handling:** Submit, accept, and track quotes with escrow funding.
-   **Dispute Resolution:** Raise disputes and execute resolution strategies including full refunds, full payments, or split payments.
-   **Secure Escrow System:** Manage funds through a PDA-based escrow mechanism ensuring security and transparency.

* * * * *

Features
--------

-   **Job Creation & Management:**\
    Clients can create detailed job postings with title, description, budget, and status flags that ensure state consistency.

-   **Quote Submission & Acceptance:**\
    Workers submit quotes against job postings, which clients can later review and accept, triggering secure fund transfers into escrow.

-   **Escrow & Payment Release:**\
    Once a quote is accepted, funds are moved to an escrow account and released only when predefined conditions (job completion and multi-party approvals) are met.

-   **Dispute Handling:**\
    Built-in dispute resolution protocols allow clients or workers to escalate issues. Arbitrators can step in to enforce a resolution, ensuring fairness and trust.

-   **Error Handling & Security:**\
    Comprehensive error management using custom error codes safeguards against unauthorized access and invalid state transitions. The use of PDAs (Program Derived Addresses) for escrow accounts further bolsters security.

* * * * *

Architecture
------------

The program is implemented in Rust using the Anchor framework, structured to promote modularity and clarity:

-   **State Modules:**

    -   `job.rs`: Defines the `Job` account structure with properties tracking state and lifecycle.
    -   `quotes.rs`: Defines the `Quote` account structure with fields for worker proposals and dispute resolutions.
-   **Instruction Modules:**\
    Each module corresponds to a key transaction:

    -   `create_job`: Initializes a new job.
    -   `update_job`: Allows modifications to existing job details.
    -   `delete_job`: Deletes jobs under specific conditions.
    -   `submit_quote`: Enables workers to propose quotes.
    -   `accept_quote`: Facilitates client acceptance and escrow funding.
    -   `raise_dispute`: Allows either party to flag issues.
    -   `release_payment`: Executes secure payment transfers.
    -   `resolve_dispute`: Handles dispute resolution strategies with flexible outcomes.
-   **Error Handling:**\
    Custom errors are implemented via the `ErrorCode` enum to provide precise feedback and ensure robust validation across transactions.

* * * * *

Getting Started
---------------

### Prerequisites

Ensure you have the following installed:

-   **Rust & Cargo:** [Installation Guide](https://www.rust-lang.org/tools/install)
-   **Anchor CLI:** [Anchor Installation](https://project-serum.github.io/anchor/getting-started/installation.html)
-   **Solana CLI:** [Solana CLI Installation](https://docs.solana.com/cli/install-solana-cli-tools)
-   **Node.js & NPM/Yarn:** Required for frontend integration (if applicable).

### Clone the Repository

```
git clone https://github.com/anubhav-auth/dework-program.git
cd dework-program

```

### Build the Program

Use the Anchor CLI to build the program:

```
anchor build

```

This command compiles the Rust code and produces the IDL and binaries required for deployment.

* * * * *

Deployment
----------

Deploy the program on your target Solana cluster (Devnet, Testnet, or Mainnet):

```
anchor deploy

```

The deployment process will register the program on the network with the following program ID:

```
92sorgqaDHqG5T12ZqrTMSCFZEHaVxrANLWdSQ5fFUom

```

Ensure your wallet has sufficient funds to cover deployment costs and follow any additional network-specific guidelines.

* * * * *

Usage
-----

After deployment, integrate the program with your frontend or use Anchor's testing utilities to interact with it. The program supports a series of transactions that can be triggered via client-side scripts or directly from the CLI:

-   **Job Management:**\
    Create, update, or delete jobs through respective instruction calls.

-   **Quote Management:**\
    Submit quotes as a worker and accept quotes as a client.

-   **Payment & Dispute Resolution:**\
    Manage payments through multi-signature approval and resolve disputes with flexible outcomes (full refund, full payment, or split payment).

For detailed instruction usage, refer to the respective modules in the repository.

* * * * *

Contract Structure
------------------

-   **`dework_program/src/lib.rs`:**\
    Main program file that declares the program, imports modules, and defines public functions.

-   **`dework_program/src/state/`:**\
    Contains the state definitions for jobs and quotes.

-   **`dework_program/src/instructions/`:**\
    Houses all the individual instruction handlers for various operations including job creation, quote submission, and dispute management.

-   **`dework_program/src/instructions/errors.rs`:**\
    Custom error definitions for robust state validation and error handling.

-   **`Anchor.toml`:**\
    Configuration file for the Anchor framework detailing build and deployment parameters.

* * * * *

Testing
-------

The program is designed to be tested using the Anchor testing framework. To run tests, execute:

```
anchor test

```

Tests validate the integrity of job management, quote handling, escrow transfers, and dispute resolutions. Ensure you review and update tests as new features are integrated.

* * * * *

Contributing
------------

Contributions are welcome from industry professionals aiming to enhance decentralized work platforms. Please adhere to the following guidelines:

-   **Code Quality:** Ensure that any contributions are thoroughly tested and follow best practices.
-   **Documentation:** Update README and inline documentation as needed.
-   **Pull Requests:** Submit clear and concise pull requests with detailed descriptions of changes.

For major changes, please open an issue first to discuss your ideas.

* * * * *

License
-------

This project is distributed under the MIT License. See [LICENSE](https://chatgpt.com/c/LICENSE) for more details.

* * * * *

Contact
-------

For inquiries or further information, please reach out via the GitHub [Issues](https://github.com/anubhav-auth/dework-program/issues) page or directly contact the repository owner.

* * * * *

The Dework Program is built to be robust, secure, and scalable. Get in touch to explore collaborative opportunities or to integrate this innovative solution into your decentralized work ecosystem.