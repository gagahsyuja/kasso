<script lang="ts">
    import Navigation from "../lib/Navigation/Navigation.svelte";
    import Main from "../lib/Main.svelte";
    import Database from "@tauri-apps/plugin-sql";
    import { Chart } from "chart.js/auto";
    import { onMount } from "svelte";
    import { fly, scale } from "svelte/transition";
    import { circIn } from "svelte/easing";
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

    const getFinalAmount = async (): Promise<number> => {

        const db = await Database.load("sqlite:database.db");

        let transactions: Array<any> = await db.select("\
            SELECT SUM(transactions.amount) AS amount\
            FROM transactions\
            WHERE transactions.date BETWEEN $1 AND $2\
            GROUP BY category_id ORDER BY amount DESC",
            [0, props.endDate]
        );

        return transactions[0].amount;
    }

    const getAllTransaction = async (type: String): Promise<Array<any>> => {

        const db = await Database.load("sqlite:database.db");

        let transactions: Array<any> = await db.select("\
            SELECT categories.name AS category, SUM(transactions.amount) AS amount\
            FROM transactions\
            LEFT JOIN categories ON transactions.category_id = categories.id\
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

                let labels = transactions.map(row => row.category ? row.category : 'Other');
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
                            labels: transactions.map(row => row.category ? row.category : 'Other'),
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
        <div class="flex justify-center align-middle pt-0">
            {#key props.startDate}
                {#await getFinalAmount() then amount}
                    <h1 class="text-2xl text-black py-2" in:fly|global={{ y: -50 }}>
                        <Currency amount={amount} bold={true} />
                    </h1>
                {/await}
            {/key}
        </div>
        <div
            class="my-4 flex flex-row justify-evenly text-xl bg-white p-2 rounded-xl w-2/3 mx-auto shadow-xl shadow-blue-100"
            in:fly={{ y: -50 }}
        >
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
        <div class="flex flex-row justify-center">
            <section class="py-2 w-4/6 h-full">
                <canvas bind:this={canvas} in:scale|global={{ duration: 500 }}></canvas>
            </section>
        </div>
    </div>
    {#key props.startDate}
        {#await getTotalAmountByMonth(selected) then amount}
            <h1 class="text-center text-lg py-2" in:fly|global={{ y: 50 }}>
                <Currency amount={amount} bold={true} /> total {selected === 'in' ? 'income' : 'expense'}
            </h1>
        {/await}
        {#await getAllTransaction(selected) then transactions}
            {#each transactions as transaction, i}
                {@const category = transaction.category ? transaction.category : 'Other'}
                <div
                    class="flex flex-row justify-between p-4 rounded-lg border-t-gray-300 border-b-2"
                    in:fly|global={{ y: 50, delay: 0 }}
                >
                    <span class="text-lg font-bold">{category}</span>
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
                    in:fly|global={{ y: 50, delay: 0 }}
                >
                    Empty
                </div>
            {/each}
        {/await}
    {/key}
</Main>
