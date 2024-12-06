const axios = require('axios');

// Введите ваш токен API Etherscan и адрес контракта токена
const apiKey = 'Y2SMGE1DMUYCRSUZPYD6XZB6AYXD9F9333';
const contractAddress = '0x1b3be8fcd2e7c5ce9c5c242e0066fdd9740220d0'; // Ваш токен LICKER
const uniswapV2Router = '0x1F98431c8aD98523631AE4a59f267346ea31F984'; // Uniswap V2 Router Address

// Получить список всех транзакций для вашего токена
async function getTokenTransactions() {
    const url = `https://api.etherscan.io/api?module=account&action=txlist&address=${contractAddress}&startblock=0&endblock=99999999&sort=asc&apikey=${apiKey}`;

    try {
        const response = await axios.get(url);
        const transactions = response.data.result;
a
        // Отфильтровать транзакции, где контракт взаимодействовал с Uniswap V2 Router
        const uniswapTransactions = transactions.filter(tx => tx.to === uniswapV2Router);

        console.log("Uniswap Transactions: ");
        console.log(uniswapTransactions);

        return uniswapTransactions;
    } catch (error) {
        console.error('Error fetching token transactions:', error);
    }
}

// Получить события для метода добавления ликвидности
async function getAddLiquidityEvents() {
    const topic0 = '0x0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9'; // Хэш метода addLiquidityETH

    const url = `https://api.etherscan.io/api?module=logs&action=getLogs&fromBlock=0&toBlock=latest&address=${uniswapV2Router}&topic0=${topic0}&apikey=${apiKey}`;

    try {
        const response = await axios.get(url);
        const events = response.data.result;

        console.log("Add Liquidity Events: ");
        console.log(events);

        return events;
    } catch (error) {
        console.error('Error fetching liquidity events:', error);
    }
}

// Основная функция для получения транзакций и событий
async function main() {
    const tokenTransactions = await getTokenTransactions();
    const liquidityEvents = await getAddLiquidityEvents();

    // Дополнительная обработка, если необходимо
}

main();
