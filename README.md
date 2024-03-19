
# Rust BigQuery Template

This template facilitates the integration of Rust applications with Google BigQuery, providing a structured approach for creating datasets, tables, inserting data, and handling errors.

## Getting Started

Follow these steps to start using the Rust BigQuery Template:

1. **Clone the Repository**: Clone this repository to your local machine:

    ```bash
    https://github.com/sheshnath-at-knoldus/BigQuery-TechHub.git
    ```

2. **Navigate to the Project Directory**: Move into the project directory:

    ```bash
    cd BigQuery-TechHub
    ```

3. **Environment Setup**:
   - Ensure Rust is installed on your system. If not, follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).
   - Create a `.env` file in the project root directory and add your Google Cloud service account key JSON:

     ```plaintext
     GCS_JSON_CREDENTIALS=your_service_account_key_json_here
     ```

4. **Configure Dependencies**:
   - Update the `Cargo.toml` file with the necessary dependencies for your project.

5. **Resource Configuration**:
   - Create a `resource` directory in the project root.
   - Inside the `resource` directory, create an `application.config` file containing the project, dataset, and table IDs:

     ```plaintext
     project_id=your_project_id
     data_set_id=your_dataset_id
     table_id=your_table_id
     ```

6. **Utility Methods**:
   - Define any utility methods required for your project in the `utility` folder.

7. **Code Implementation**:
   - Customize the provided template code in `src/main.rs`, `src/big_query.rs`, `src/model.rs`, and `src/utility` to fit your specific use case.
   - Replace placeholders like `CONFIG.project_id`, `CONFIG.data_set_id`, and `CONFIG.table_id` with your actual project, dataset, and table IDs.

8. **Build and Run**:
   - Build and run your Rust project using Cargo:

     ```bash
     cargo run
     ```

## Project Structure

- **src/main.rs**: Entry point of the application. Initializes the environment, GCP credentials, and orchestrates dataset, table creation, and data insertion.
- **src/big_query.rs**: Contains functions for interacting with BigQuery, such as creating datasets, tables, and inserting data.
- **src/model.rs**: Defines the data model used for inserting data into BigQuery.
- **src/utility**: Handles configuration loading from a `.conf` file and environment variable retrieval.

## Customization

- Customize the provided template according to your project requirements.
- Ensure proper error handling and logging are implemented for production use.
- Adapt the code to handle different data types or additional BigQuery operations as needed.


## Acknowledgements

- This template is inspired by the need for integrating Rust applications with Google BigQuery.
- Thanks to the Rust community for providing valuable resources and support.

---

Feel free to modify and enhance this template as required for your projects. If you encounter any issues or have suggestions for improvement, please open an issue on GitHub. Happy coding!

