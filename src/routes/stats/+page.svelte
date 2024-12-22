<script lang="ts">
    import Navigation from "../lib/Navigation/Navigation.svelte";
    import Main from "../lib/Main.svelte";
    import Database from "@tauri-apps/plugin-sql";
    import { Chart } from "chart.js/auto";
    import { onMount } from "svelte";
    import { fly, fade } from "svelte/transition";
    import Currency from "../lib/Currency.svelte";
    import Calendar from "../lib/Calendar.svelte";

    let canvas: HTMLCanvasElement;
    let selected = $state('in');
    let chart: Chart;
    let ready = false;

    let date = new Date();

    let startDate = Date.parse(`${date.getMonth() + 1}/01/${date.getFullYear()}`);
    startDate = parseInt(startDate.toString());

    let endDate = Date.parse(`${date.getMonth() + 1}/31/${date.getFullYear()}`);
    endDate = parseInt(endDate.toString());

    let props = $state({
        startDate,
        endDate
    });

    let backgroundColor = [
        '#003A6B',
        '#1B5886',
        '#3776A1',
        '#5293BB',
        '#6EB1D6',
        '#89CFF1'
    ];

    const getAllTransaction = async (type: String): Promise<Array<any>> => {

        const db = await Database.load("sqlite:database.db");

        let transactions: Array<any> = await db.select("\
            SELECT categories.name AS category, SUM(transactions.amount) AS amount\
            FROM transactions\
            INNER JOIN categories ON transactions.category_id = categories.id\
            WHERE transactions.type = $1\
            AND transactions.date BETWEEN $2 AND $3\
            GROUP BY category_id ORDER BY amount DESC",
            [type, props.startDate, props.endDate]
        );

        return transactions;
    }

    const getTotalAmountByMonth = async (type: String): Promise<number> => {

        const db = await Database.load("sqlite:database.db");

        let amount: Array<any> = await db.select("\
            SELECT SUM(transactions.amount) AS amount\
            FROM transactions\
            INNER JOIN categories ON transactions.category_id = categories.id\
            WHERE transactions.type = $1\
            AND transactions.date BETWEEN $2 AND $3",
            [type, props.startDate, props.endDate]
        );

        return amount[0].amount;
    }

    const generateChart = async (type: String) => {

        if (ready) {

            let transactions: Array<any> = await getAllTransaction(type);

            if (transactions.length) {

                let labels = transactions.map(row => row.category);
                let data = transactions.map(row => row.amount);

                let datasets = [
                    {
                        labels: 'Total amount',
                        data,
                        backgroundColor
                    }
                ];

                chart.data.labels = labels;
                chart.data.datasets = datasets;

            } else {

                let datasets = [
                    {
                        data: [100],
                        backgroundColor: '#B5B5B5'
                    }
                ];

                chart.data.labels = [];
                chart.data.datasets = datasets;

            }

            chart.update();
        }
    };

    $effect(() => {
        props.startDate;
        props.endDate;
        generateChart(selected);
    })

    onMount(() => {

        getAllTransaction(selected)
            .then(transactions => {
                
                if (transactions.length) {
                    chart = new Chart(canvas, {
                        type: 'pie',
                        data: {
                            labels: transactions.map(row => row.category),
                            datasets: [
                                {
                                    label: 'Total amount',
                                    data: transactions.map(row => row.amount),
                                    backgroundColor
                                }
                            ]
                        }
                    });
                } else {
                    chart = new Chart(canvas, {
                        type: 'pie',
                        data: {
                            labels: [],
                            datasets: [
                                {
                                    data: [100],
                                    backgroundColor: '#B5B5B5'
                                }
                            ]
                        }
                    });
                }

                ready = true;
            });

    })
</script>

<Navigation />

<Main>
    <div class="sticky top-4 bg-blue-50">
        <!-- <Title title="Stats" /> -->
        <Calendar bind:props />
        <div class="flex flex-row justify-evenly text-xl bg-white p-4 rounded-xl w-5/6 mx-auto shadow-xl shadow-blue-100">
            <button
                onclick={() => selected = 'in'}
                class={`${selected === 'in' ? 'text-blue-800' : 'text-black'}`}
            >
                <span>Income</span>
            </button>
            <button
                onclick={() => selected = 'out'}
                class={`${selected === 'out' ? 'text-blue-800' : 'text-black'}`}
            >
                Expense
            </button>
        </div>
        <div class="flex justify-center align-middle pt-4">
            {#key props.startDate}
                {#await getTotalAmountByMonth(selected) then amount}
                    <h1 class="text-2xl text-black py-2" in:fly|global={{ delay: 470 }}>
                        <Currency amount={amount} bold={true} />
                    </h1>
                {/await}
            {/key}
        </div>
        <div class="flex flex-row justify-center" in:fly|global={{ delay: 0 }}>
            <section class="py-2 w-4/6 h-full">
                <canvas bind:this={canvas}></canvas>
            </section>
        </div>
    </div>
    {#key props.startDate}
        {#await getAllTransaction(selected) then transactions}
            {#each transactions as transaction, i}
                <div
                    class="flex flex-row justify-between p-4 rounded-lg border-t-gray-300 border-b-2"
                    in:fly|global={{ y: 50, delay: 50 }}
                >
                    <span class="text-lg font-bold">{transaction.category}</span>
                    <span class="text-md">
                        {#if selected === 'out'}
                            -
                        {/if}
                        <Currency amount={transaction.amount} bold={true} subUnit={false} />
                    </span>
                </div>
            {:else}
                <div
                    class="text-center font-normal text-xl"
                    in:fly|global={{ y: 50, delay: 50 }}
                >
                    Empty
                </div>
            {/each}
        {/await}
    {/key}
</Main>
