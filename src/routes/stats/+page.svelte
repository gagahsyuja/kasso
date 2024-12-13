<script lang="ts">
    import Navigation from "../lib/Navigation/Navigation.svelte";
    import Main from "../lib/Main.svelte";
    import Database from "@tauri-apps/plugin-sql";
    import { Chart } from "chart.js/auto";
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
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

    const getAllTransaction = async (type: String): Promise<Array<any>> => {

        const db = await Database.load("sqlite:database.db");

        let transactions: Array<any> = await db.select("\
            SELECT categories.name AS category, SUM(transactions.amount) AS amount\
            FROM transactions\
            INNER JOIN categories ON transactions.category_id = categories.id\
            WHERE transactions.type = $1\
            AND transactions.date BETWEEN $2 AND $3\
            GROUP BY category_id",
            [type, props.startDate, props.endDate]
        );

        return transactions;
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
                        data
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
                
                chart = new Chart(canvas, {
                    type: 'pie',
                    data: {
                        labels: transactions.map(row => row.category),
                        datasets: [
                            {
                                label: 'Total amount',
                                data: transactions.map(row => row.amount),
                            },
                        ],
                    },
                });

                ready = true;
            });

    })
</script>

<Navigation />

<Main>
    <div class="sticky top-4 bg-white">
        <!-- <Title title="Stats" /> -->
        <Calendar bind:props />
        <div class="flex flex-row justify-evenly text-2xl bg-gray-200 p-4 rounded-xl w-[400px] mx-auto">
            <button onclick={() => selected = 'in'}>
                Income
            </button>
            <button onclick={() => selected = 'out'}>
                Expense
            </button>
        </div>
        <div class="flex flex-row justify-center">
            <section class="py-10 w-[300px] h-full">
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
                    <span class="text-xl font-bold">{transaction.category}</span>
                    <span class="text-lg">
                        {#if selected === 'out'}
                            -
                        {/if}
                        <Currency amount={transaction.amount} bold={true} />
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
