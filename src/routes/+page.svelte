<script lang="ts">
    import { onMount } from "svelte";
    import Database from "@tauri-apps/plugin-sql";
    import Navigation from "$lib/Navigation/Navigation.svelte";
    import ListTransaction from "$lib/ListTransaction.svelte";
    import Title from "$lib/Title.svelte";
    import Main from "$lib/Main.svelte";
    import Fa from "svelte-fa";
    import Currency from "$lib/Currency.svelte";
    import Empty from "$lib/Empty.svelte";
    import { faArrowTrendUp, faArrowTrendDown, faBell } from "@fortawesome/free-solid-svg-icons";
    import { fly } from "svelte/transition";
    import { goto } from "$app/navigation";

    interface Transaction {
        id: number,
        amount: number,
        type: string,
        method: string,
        date: number,
        description: string,
        category_id: number
    };

    let showModal = $state(false);
    let object = $state({
        last: [{} as Transaction],
        totalIn: 0,
        totalOut: 0
    });

    $effect(() => {
        showModal;
        load();
    })

    let currentPage = $state('home');

    const load = async () => {

        const db = await Database.load("sqlite:database.db");

        let last: Array<any> = await db.select("SELECT * FROM transactions ORDER BY date DESC LIMIT 3");
        let totalIn: Array<any> = await db.select("SELECT amount FROM transactions WHERE type = ?", ['in']);
        let totalOut: Array<any> = await db.select("SELECT amount FROM transactions WHERE type = ?", ['out']);

        object.totalIn = totalIn.reduce((total, num) => {
            return total + num.amount;
        }, 0) as number;

        object.totalOut = totalOut.reduce((total, num) => {
            return total + num.amount;
        }, 0) as number;

        object.last = last ? last : [{}];
    }

    const checkNotification = async (): Promise<boolean> => {

        let username = localStorage.getItem('username');

        const db = await Database.load("sqlite:database.db");

        const status: Array<any> = await db.select(
            "SELECT new_notification FROM users WHERE username = $1",
            [ username ]
        );

        return status.length
            ? Boolean(status[0].new_notification)
            : false;
    };

    const setup = async () => {
        
        const db = await Database.load("sqlite:database.db");

        const exist: Array<object> = await db.select("SELECT * FROM categories WHERE name = $1", ['Other']);

        if (!exist.length) {
            await db.execute("INSERT INTO categories (name, type) VALUES ($1, $2)", [ 'Other', 'in' ]);
            await db.execute("INSERT INTO categories (name, type) VALUES ($1, $2)", [ 'Other', 'out' ]);
        }
    }

    onMount(async () => {

        if (!localStorage.getItem("username")) goto('/login', { replaceState: true });

        await setup();

    });
</script>

<Main>
    {#await load() then}
        <div class="flex flex-row justify-between items-center">
            <div class="flex flex-row justify-start items-center">
                <Title title="Hello, {localStorage.getItem('username')}!" />
                <div class="rounded-lg px-2 font-bold border-2 bg-blue-200 border-blue-900 text-xs text-blue-900"
                    in:fly|global={{ y: -50 }}
                >
                    {localStorage.getItem('role') === 'member' ? "Member" : "Treasurer"}
                </div>
            </div>
            {#key showModal}
                <a href="/notification" in:fly={{ x: 50, y: -50 }}>
                    {#await checkNotification() then exist}
                        <div class="{ exist ? "zoom" : ""}">
                            <Fa icon={faBell} size="1.35x" class="px-4" />
                        </div>
                    {/await}
                </a>
            {/key}
        </div>
        <div class="flex flex-col justify-center bg-blue-900 rounded-2xl shadow-xl shadow-blue-100" in:fly|global={{ y: 50, x: -50 }}>
            <div class="flex flex-col justify-center items-center">
                <div class="flex flex-col justify-center items-center py-4 px-4">
                    <h1 class="text-md font-bold text-gray-300">
                        Total Balance
                    </h1>
                    <h1 class="text-2xl text-white font-bold">
                        <Currency amount={object.totalIn - object.totalOut} bold={true} />
                    </h1>
                </div>
                <hr class="w-4/5" />
                <div class="flex flex-row justify-center items-center pb-4 pt-1 flex-wrap space-y-0">
                    <div class="flex flex-col justify-center items-center px-4 py-3">
                        <div class="flex flex-row justify-center items-center">
                            <div class="p-4 bg-white rounded-xl w-12 h-12 flex justify-center items-center">
                                <Fa icon={faArrowTrendUp} size="1.2x" />
                            </div>
                            <div class="flex flex-col justify-center items-start ml-2">
                                <h1 class="text-sm font-normal text-gray-300">
                                    Income
                                </h1>
                                <h1 class="text-md text-white">
                                    <Currency amount={object.totalIn} bold={true} subUnit={false} />
                                </h1>
                            </div>
                        </div>
                    </div>
                    <div class="flex flex-col justify-center items-center px-4">
                        <div class="flex flex-row justify-center items-center self-start min-w-32">
                            <div class="p-4 bg-white rounded-xl w-12 h-12 flex justify-center items-center">
                                <Fa icon={faArrowTrendDown} size="1.2x" />
                            </div>
                            <div class="flex flex-col items-start ml-2">
                                <h1 class="text-sm font-normal text-gray-300">
                                    Expense
                                </h1>
                                <h1 class="text-md text-white">
                                    <Currency amount={object.totalOut} bold={true} subUnit={false} />
                                </h1>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <!-- <div class="flex flex-row justify-center items-center h-28 my-2"> -->
        <!--     <a -->
        <!--         href="/login" -->
        <!--         class="bg-white grow min-h-24 rounded-3xl flex justify-center items-center shadow-xl shadow-blue-100"> -->
        <!--         <h1 class="text-3xl font-bold">Monthly Report</h1> -->
        <!--     </a> -->
        <!--     <a href="/dates" class="bg-gray-300 p-4 min-h-24 min-w-24 rounded-3xl ml-4 flex justify-center items-center"> -->
        <!--         <Fa icon={faPlus} size="2x" /> -->
        <!--     </a> -->
        <!-- </div> -->
        <div>
            <div class="flex flex-row justify-between items-center" in:fly|global={{ y: 50, delay: 0 }}>
                <h1 class="text-black text-2xl p-4 font-bold">Overview</h1>
                <a href="/history" class="text-blue-900 text-md p-4 font-bold">See More</a>
            </div>
            <div class="flex flex-col space-y-4" in:fly|global={{ y: 50, delay: 0 }}>
                {#each object.last as obj}
                    <ListTransaction
                        id={obj.id}
                        amount={obj.amount}
                        type={obj.type}
                        method={obj.method}
                        date={obj.date}
                        description={obj.description}
                        categoryId={obj.category_id}
                    />
                {:else}
                    <Empty value="No recent transaction." />
                {/each}
            </div>
        </div>
    {/await}
</Main>

{#key currentPage}
    <Navigation bind:showModal />
{/key}

<style>
    .zoom {
        animation: zoom-in-out 1s ease-in-out infinite;
    }
    @keyframes zoom-in-out {
        0% {
            scale: 100%;
            color: black;
        }
        25% {
            color: #1e3a8a;
        }
        50% {
            scale: 130%;
            color: #1e3a8a;
        }
        100% {
            scale: 100%;
            color: black;
        }
    }
</style>
