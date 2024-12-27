<script lang="ts">
    import HugeButton from "../HugeButton.svelte";
    import FilterItem from "../FilterItem.svelte";
    import Database from "@tauri-apps/plugin-sql";
    import { fly } from "svelte/transition";
    import { onMount } from "svelte";

    let { showNotification = $bindable() } = $props();

    let amount = $state(0);

    const handleChangeNotify = async () => {

        let username = localStorage.getItem('username');

        if (username) {

            const db = await Database.load("sqlite:database.db");

            await db.execute(
                "UPDATE users SET notify = $1 WHERE username = $2",
                [ amount, username ]
            );

        }

        localStorage.setItem('notify', amount.toString());

        showNotification = false;
    };

    onMount(async () => {
        
        const db = await Database.load("sqlite:database.db");

        let currentlySet: Array<any> = await db.select(
            "SELECT notify FROM users WHERE username = $1 LIMIT 1",
            [ localStorage.getItem('username') ]
        );

        if (currentlySet.length) amount = currentlySet[0].notify
    })

</script>

<div class="w-full h-full bg-blue-900/95 top-0 left-0 fixed z-[999]" in:fly={{ y: 50, duration: 0 }}>
    <div class="fixed z-[999] inset-0 top-20 mx-auto
        p-5 border w-[90%] h-1/2 rounded-xl bg-white flex
        flex-col space-y-2 justify-center"
        in:fly|global={{ y: 50, duration: 100 }} out:fly|global={{ y: -50, duration: 100 }}
    >
        <div class="flex flex-row justify-between items-center scrollbar-hide pt-2">
            <span class="text-xl font-bold">Edit Notification</span>
            <button class="text-md text-blue-900 font-bold" onclick={() => showNotification = false}>Cancel</button>
        </div>
        <div class="flex flex-col justify-center items-start space-x-2">
            <span class="font-bold p-2">Notify me at</span>
            <div class="flex flex-row items-center space-x-2">
                <input
                    type="number"
                    class="w-full border-2 border-blue-100 shadow-xl shadow-blue-100 focus:ring-0 rounded-xl"
                    bind:value={amount}
                />
            </div>
        </div>
        <HugeButton value="Add" onclick={handleChangeNotify} />
    </div>
</div>
