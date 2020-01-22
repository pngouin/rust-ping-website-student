import * as wasm from 'client';

const SERVER_URL = 'http://localhost:8000'

const pingRequestForm = document.forms['ping-request-form'];
const resultContainer = document.getElementById('result-container');

const displayResult = (domain, iterations, ping) => {
    const paragraph = document.createElement('p');
    paragraph.textContent = `${domain} :: ${ping} ms avg. on ${iterations} request(s)`;
    resultContainer.appendChild(paragraph);
}

const pingServer = async (ev) => {
    ev.preventDefault();

    const inputs = pingRequestForm.getElementsByTagName('input');
    const domain = inputs[0].value;
    const iterations = inputs[1].value;

    const encodedDomain = encodeURIComponent(domain);
    const encodedIterations = encodeURIComponent(iterations);
    const body = `domain=${encodedDomain}&iterations=${encodedIterations}`;

    const url = `${SERVER_URL}/ping?${body}`;

    const xhr = new XMLHttpRequest();
    xhr.open('POST', url, true);
    
    xhr.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');

    xhr.onreadystatechange = function() {
        if (this.readyState === 4 && this.status === 200) {
            const response = JSON.parse(this.response);
            displayResult(domain, iterations, response.ping);
        }
    }

    xhr.send(body);

    return false;
}

pingRequestForm.addEventListener('submit', pingServer);