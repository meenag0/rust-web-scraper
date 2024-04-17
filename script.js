// Function to get the dates for this week (Sunday to Saturday)
function getThisWeekDates() {
    let today = new Date();
    let currentDay = today.getDay();
    let diff = today.getDate() - currentDay + (currentDay === 0 ? -6 : 1); // Adjust if today is Sunday
    let startDate = new Date(today.setDate(diff));
    let endDate = new Date(today.setDate(startDate.getDate() + 6)); // Next Saturday

    return {
        startDate,
        endDate
    };
}

// Function to format the date as "Month Day"
function formatDate(date) {
    const options = { month: 'short', day: 'numeric' };
    return date.toLocaleDateString('en-US', options);
}

// Get this week's dates
let thisWeekDates = getThisWeekDates();

// Inserting text into the HTML
let readingListDiv = document.getElementById("reading-list");
let text = document.createTextNode("Reading List for {" + formatDate(thisWeekDates.startDate) + " - " + formatDate(thisWeekDates.endDate) + "}");
readingListDiv.appendChild(text);


window.onload = function() {
    fetch('http://localhost:8080/articles')
    .then(response => {
      if (!response.ok) {
        throw new Error('Network response was not ok');
      }
      return response.json();
    })
    .then(data => {
      // Handle the JSON response data
      console.log(data);
      
      // Get the container element where you want to display the articles
      let container = document.getElementById('articles-container');
      
      // Iterate over the data and create HTML elements for each article
      data.forEach(article => {
        // Create a <div> element for the article
        let articleDiv = document.createElement('div');
        
        // Set the class of the <div> element
        articleDiv.className = 'article';
        
        // Create a <h3> element for the title
        let titleElement = document.createElement('h3');
        titleElement.textContent = article.title;
        
        // Create a <p> element for the author
        let authorElement = document.createElement('p');
        authorElement.textContent = '' + article.author;
        
        // Append the title and author elements to the article <div>
        articleDiv.appendChild(titleElement);
        articleDiv.appendChild(authorElement);
        
        // Append the article <div> to the container
        container.appendChild(articleDiv);
      });
    })
    .catch(error => {
      console.error('There was a problem with the fetch operation:', error);
    });
};
