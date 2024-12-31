<script lang="ts">
    import Fa from "svelte-fa";
    import { fade } from "svelte/transition";
    import { faArrowLeft, faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
    import Currency from "./Currency.svelte";
    import Database from "@tauri-apps/plugin-sql";

    let { props = $bindable() } = $props();

    let currentMonth = $state(0);
    let currentYear = $state(0);

    const setDate = (month: number, year: number) => {
        
        let startDate = Date.parse(`${month + 1}/01/${year}`);

        // let endDate = Date.parse(`${month + 1}/31/${year}`);
        // let endDate: Date | number = new Date(year, month + 1, 1, 0, 23, 59, 59);
        let endDate: Date | number = new Date(year, month + 1, 0, 23, 59, 59);
        console.log(year, month);
        endDate = endDate.getTime();

        props.startDate = startDate;
        props.endDate = endDate;
    };

    const increaseMonth = () => {

        if (currentMonth === 11) currentYear += 1;

        currentMonth += 1;
        currentMonth %= 12;

        setDate(currentMonth, currentYear);
    }

    const decreaseMonth = () => {

        currentMonth -= 1;

        if (currentMonth < 0) {
            currentMonth = 11;
            currentYear -= 1;
        }

        setDate(currentMonth, currentYear);
    }

    const months = [
        'January',
        'February',
        'March',
        'April',
        'May',
        'June',
        'July',
        'August',
        'September',
        'October',
        'November',
        'December'
    ];

    const getFinalAmount = async (): Promise<number> => {

        const db = await Database.load("sqlite:database.db");

        let transactions: Array<any> = await db.select("\
            SELECT SUM(transactions.amount) AS amount\
            FROM transactions\
            WHERE transactions.date BETWEEN $1 AND $2\
            ORDER BY amount DESC",
            [0, props.endDate]
        );

        return transactions.length
            ? transactions[0].amount
            : 0;
    }

    onMount(() => {

        let date = new Date(props.startDate);

        currentMonth = date.getMonth();
        currentYear = date.getFullYear();

        setDate(currentMonth, currentYear);
    })
</script>

<div class="flex flex-row items-center justify-between text-xl pb-2 pt-0 w-11/12 mx-auto">
    <button
        class="flex flex-row items-center justify-center px-4 w-[45px] h-[45px] bg-blue-900 rounded-lg text-white"
        onclick={() => decreaseMonth()}
        in:fly={{ x: -50, y: -50 }}
    >
        <Fa icon={faArrowLeft} />
    </button>
    {#key currentMonth}
    <div class="flex flex-col justify-center items-center">
        <div class="space-x-2 font-bold flex flex-row justify-center items-center text-md" in:fly={{ y: -50 }}>
            <span>{months[currentMonth]}</span>
            <span class="font-normal">{currentYear}</span>
        </div>
        <div class="flex justify-center align-middle pt-0">
            {#await getFinalAmount() then amount}
                <h1 class="text-xl text-black py-2" in:fly|global={{ y: -50 }}>
                    <Currency amount={amount} bold={true} />
                </h1>
            {/await}
        </div>
    </div>
    {/key}
    <button
        class="flex flex-row items-center justify-center px-4 w-[45px] h-[45px] bg-blue-900 rounded-lg text-white"
        onclick={() => increaseMonth()}
        in:fly={{ x: 50, y: -50 }}
    >
        <Fa icon={faArrowRight} />
    </button>
</div>
