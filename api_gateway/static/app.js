document.addEventListener('DOMContentLoaded', function () {
    const apiUrl = 'http://127.0.0.1:8000'; // Adjust this URL to where your API is hosted
  
    // Function to check activation status
    function checkActivation(workerName, password) {
        fetch(`${apiUrl}/check_activation/${workerName}/${password}`)
            .then(response => {
                if (response.ok) return response.json();
                throw new Error('Network response was not ok.');
            })
            .then(data => {
                if (data) {
                    document.getElementById('dashboardSection').style.display = 'block';
                    document.getElementById('loginSection').style.display = 'none';
                    document.getElementById('registerSection').style.display = 'none';
                } else {
                    alert('This account is not activated.');
                }
            })
            .catch(error => console.error('Failed to fetch:', error));
    }



    // Function to register a new miner
    function registerMiner(newProfile) {
        fetch(`${apiUrl}/add_miner_profile`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(newProfile)
        })
            .then(response => {
                if (response.status === 201) {
                    alert('Registration successful. Please login.');
                } else {
                    throw new Error('Failed to create profile.');
                }
            })
            .catch(error => alert('Error during registration:', error));
    }

    document.getElementById('loginButton').onclick = function () {
        const username = document.getElementById('loginUsername').value;
        const password = document.getElementById('loginPassword').value;
        checkActivation(username, password);
    };

    document.getElementById('registerButton').onclick = function () {
        const username = document.getElementById('registerUsername').value;
        const password = document.getElementById('registerPassword').value;
        const payoutAddress = document.getElementById('registerPayoutAddress').value;

        const newProfile = {
            worker_name: username,
            password: password,
            is_activated: false, // Assume all new registrations are initially not activated
            payout_address: payoutAddress,
            payment_scheme: 'PPLNS',
        };
        registerMiner(newProfile);
    };
});

const userStats = {
    hashrate: '4.5 TH/s',
    validShares: 12345,
    invalidShares: 23,
    staleShares: 10,
    estimatedEarnings: '0.00001234 BTC',
    confirmedEarnings: '0.02345678 BTC',
    unconfirmedEarnings: '0.00001234 BTC',
    lastPayment: '0.00123456 BTC',
    totalPayments: '0.04567890 BTC',
    activeWorkers: 5,
    inactiveWorkers: 1
};

function fetchData() {
    // Simulating an API call with setTimeout
    setTimeout(() => {
        displayStats(userStats);
    }, 500); // Simulate network delay
}

function displayStats(stats) {
    const container = document.getElementById('statsContainer');
    container.innerHTML = `
        <p>Hashrate: ${stats.hashrate}</p>
        <p>Valid Shares: ${stats.validShares}</p>
        <p>Invalid Shares: ${stats.invalidShares}</p>
        <p>Stale Shares: ${stats.staleShares}</p>
        <p>Estimated Earnings: ${stats.estimatedEarnings}</p>
        <p>Confirmed Earnings: ${stats.confirmedEarnings}</p>
        <p>Unconfirmed Earnings: ${stats.unconfirmedEarnings}</p>
        <p>Last Payment: ${stats.lastPayment}</p>
        <p>Total Payments: ${stats.totalPayments}</p>
        <p>Active Workers: ${stats.activeWorkers}</p>
        <p>Inactive Workers: ${stats.inactiveWorkers}</p>
    `;
}
