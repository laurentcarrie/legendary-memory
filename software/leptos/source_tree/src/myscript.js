export function xxx() {

    const listItems = ['Item 1', 'Item 2', 'Item 3', 'Item 4', 'Item 5'];

    const ul = document.getElementById('myList');
    const toggleBtn = document.getElementById('toggleBtn');

    // Dynamically create list items
    listItems.forEach(item => {
        const li = document.createElement('li');
        li.textContent = item;
        ul.appendChild(li);
    });

    // Toggle button click event
    toggleBtn.addEventListener('click', () => {
        if (ul.style.display === 'none') {
            ul.style.display = 'block';
            toggleBtn.textContent = 'Fold List';
        } else {
            ul.style.display = 'none';
            toggleBtn.textContent = 'Expand List';
        }
    });
}
