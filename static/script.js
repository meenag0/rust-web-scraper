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
    fetch('/api')
    .then(response => {
      if (!response.ok) {
        throw new Error('Network response was not ok');
      }
      return response.json();
    })
    .then(data => {

      console.log(data);
      
      let container = document.getElementById('articles-container');
      
      // Iterate over the data and create HTML elements for each article
      data.forEach(article => {

        let articleDiv = document.createElement('div');
        
        articleDiv.className = 'article';
        
        let titleElement = document.createElement('h3');
        titleElement.textContent = article.title;
        
        let authorElement = document.createElement('p');
        authorElement.textContent = '' + article.author;
        
        articleDiv.appendChild(titleElement);
        articleDiv.appendChild(authorElement);
        
        container.appendChild(articleDiv);
      });
    })
    .catch(error => {
      console.error('There was a problem with the fetch operation:', error);
    });
};
