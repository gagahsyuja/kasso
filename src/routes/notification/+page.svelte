<script lang="ts">
    import Navigation from "../lib/Navigation/Navigation.svelte";
    import Main from "../lib/Main.svelte";
    import Title from "../lib/Title.svelte";
    import NotificationItem from "../lib/NotificationItem.svelte";
    import Database from "@tauri-apps/plugin-sql";
    import Empty from "../lib/Empty.svelte";
    import { onMount } from "svelte";

    const getNotification = async (): Promise<Array<any>> => {

        let notify: number = localStorage.getItem('notify')
                        ? parseInt(localStorage.getItem('notify')!)
                        : 500000;

        const db = await Database.load("sqlite:database.db");
        
        const result: Array<any> = await db.select(
            "SELECT * FROM transactions WHERE amount >= $1 ORDER BY date ASC",
            [ notify ]
        );

        return result;
    };

    const updateNotificationStatus = async () => {

        let username = localStorage.getItem('username');

        const db = await Database.load("sqlite:database.db");
        
        await db.execute(
            "UPDATE users SET new_notification = 0 WHERE username = $1",
            [ username ]
        );
    };

    onMount(async () => {
        await updateNotificationStatus()
    })
</script>

<Navigation />
<Main>
    <Title title="Notifications" />
    <div class="flex flex-col justify-center items-stretch space-y-4">
        {#await getNotification() then transactions}
            {#each transactions as transaction}
                <NotificationItem
                    edit={false}
                    amount={transaction.amount}
                    type={transaction.type}
                    date={transaction.date}
                    description={transaction.description}
                />
            {:else}
                <Empty value="No notifications yet!" />
            {/each}
        {/await}
    </div>
</Main>
