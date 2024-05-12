# readthisweek

**readthisweek** is a lightweight web application that displays a curated reading list for the current week. The app leverages a rust backend, and front-end design using HTML, CSS, and JavaScript to fetch and present articles dynamically.

## Project Structure

### Frontend

- **index.html**: The main HTML file that serves as the base structure of the webpage. Key components:
  - A header that displays the reading list dates.
  - A content area that displays dynamically fetched articles.

- **style.css**: Defines the layout and appearance of the web page, focusing on:
  - Responsive design for a consistent look across devices.
  - Styling for headers, article cards, and other HTML elements.

- **script.js**: JavaScript file that implements the logic for fetching articles and updating the UI.
  - Fetches articles using a `/api` endpoint.
  - Dynamically updates the DOM with the retrieved articles.
  - Displays the current week's reading list dates.

### Backend

- **main.rs**: Rust file (or any server file that may be used) that provides the backend server.
  - Handles requests from the frontend via REST API endpoints.
  - Processes and returns JSON data for articles based on the `/api` request.

## Features

- **Reading List**: Displays a curated list of articles for the current week.
- **Dynamic Article Fetching**: Uses JavaScript to dynamically fetch and display articles.
- **Responsive Design**: Adapts to different screen sizes for consistent user experience.
