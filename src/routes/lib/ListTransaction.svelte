<script lang="ts">
    import { formatDistanceToNow } from 'date-fns';
    import Fa from 'svelte-fa';
    import { faArrowUp, faArrowDown } from '@fortawesome/free-solid-svg-icons';
    import Database from "@tauri-apps/plugin-sql";
    import DetailTransaction from './DetailTransaction.svelte';
    import Currency from './Currency.svelte';

    let {
        edit = false,
        amount = 69,
        type = 'in',
        date = 1733602382348,
        description = 'Description goes here',
        categoryId = 0
    } = $props();

    let dateObj = $state(new Date(date));
    let category = $state();

    let data = $state({
        showDetailTransaction: false,
        amount,
        type,
        date,
        description,
        categoryId
    });

    $effect(() => {
        Database.load("sqlite:database.db").then((db: Database) => {
            db.select(
                "SELECT name FROM categories WHERE id=?",
                [categoryId]
            ).then((categories: any) => {
                category = categories.length !== 0
                    ? categories[0].name
                    : 'Transaction type';
            });
        });

        data = {
            showDetailTransaction: false,
            amount,
            type,
            date,
            description,
            categoryId
        };
    });
</script>

{#if data.showDetailTransaction}
    <DetailTransaction
        { data }
    />

{/if}

<button
    onclick={() => {
        if (!edit) {data.showDetailTransaction = true}}
    }
    class="flex flex-row justify-between items-center bg-white shadow-xl shadow-blue-100
        px-4 py-4 rounded-xl w-full text-left"
>
    <div class="flex flex-row">
        <div class="flex flex-row items-center 
            {type === 'in' ? "bg-blue-100" : "bg-blue-100"}
            w-[50px] h-[50px] p-4 rounded-full
        ">
            {#if type === 'in'}
                <Fa icon={faArrowDown} size="1.5x"/>
            {:else}
                <Fa icon={faArrowUp} size="1.5x"/>
            {/if}
        </div>
        <div class="flex flex-col pl-4">
            <span class="font-bold text-lg capitalize">{category}</span>
            <span class="text-sm">{formatDistanceToNow(dateObj, { addSuffix: true })}</span>
        </div>
    </div>
    <span class="text-lg">
        {#if type === 'out'}
            -
        {:else}
            +
        {/if}
        <Currency amount={amount} bold={true} subUnit={false} />
    </span>
</button>

<style>
    :global(html) {
        background-color: rgb(239 246 255);
    }
</style>
