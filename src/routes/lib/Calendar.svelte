<script lang="ts">
    import Fa from "svelte-fa";
    import { fade } from "svelte/transition";
    import { faArrowLeft, faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import { onMount } from "svelte";

    let { props = $bindable() } = $props();

    let currentMonth = $state(0);
    let currentYear = $state(0);

    const setDate = (month: number, year: number) => {
        
        let startDate = Date.parse(`${month + 1}/01/${year}`);

        let endDate = Date.parse(`${month + 1}/31/${year}`);

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

    onMount(() => {

        let date = new Date(props.startDate);

        currentMonth = date.getMonth();
        currentYear = date.getFullYear();
    })
</script>

<div class="flex flex-row items-center justify-between text-3xl pb-8 pt-4 w-[400px] mx-auto">
    <button
        class="flex flex-row items-center justify-center px-4 w-[50px] h-[50px] bg-red-500 rounded-lg text-white"
        onclick={() => decreaseMonth()}
    >
        <Fa icon={faArrowLeft} />
    </button>
    {#key currentMonth}
        <div class="px-4 font-bold" in:fade={{ delay: 0 }}>
            {months[currentMonth]}
            <span class="font-normal">{currentYear}</span>
        </div>
    {/key}
    <button
        class="flex flex-row items-center justify-center px-4 w-[50px] h-[50px] bg-red-500 rounded-lg text-white"
        onclick={() => increaseMonth()}
    >
        <Fa icon={faArrowRight} />
    </button>
</div>
