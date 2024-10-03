# API Response Validator Tool

## Overview

The API Response Validator Tool is a command-line utility designed to facilitate comparing API responses between two versions of your backend infrastructure. It helps ensure that the changes made in the database infrastructure do not inadvertently affect the API responses. The tool generates and compares mock API responses, providing insights into any discrepancies between the versions of HTTP Archive (HAR).

## Installation

To install the API Response Validator Tool, follow these steps:

1. Clone this repository to your local machine.
2. Install the necessary dependencies by running `npm install`.
3. Ensure that you have Node.js and git installed on your machine.
4. Install `har-to-mocks`

```bash
npm install -g @sbimochan/har-to-mocks
```

## Usage

### Generating Mock API Responses

Before comparing the API responses, you need to generate mock responses . Follow these steps:

1. Ensure that you have the necessary configuration files for the source and target versions (e.g., `source.har` and `target.har`).
2. Download and put the HAR file over here between two versions of app. Check youtube video how to export HAR from network tab

### Comparing API Responses

Once the mock API responses are generated, you can compare them to identify any differences. Follow these steps:

1. Run the following command to compare the API responses:

   ```
   npm run diff
   ```

   This command will compare the mock responses generated for the source and target versions. Any differences in the responses will be highlighted, providing insights into the impact of database infrastructure changes on API responses.

### Viewing Differences in VS Code

To view the differences in the API responses directly in VS Code's source control:

1. Ensure that you have the Git extension installed in VS Code.
2. After running the comparison command, open the VS Code source control view.
3. You should see the changes in the `source` and `target` folders, indicating the differences between the API responses of the two versions.

### Final step

After being satisfied with the result, hit

```
npm run flush
```

## Contributing

Contributions to the API Response Validator Tool are welcome! If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---
