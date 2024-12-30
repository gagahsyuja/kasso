<script lang="ts">
    import { faBook, faChartPie, faGear, faClockRotateLeft, faPlus } from "@fortawesome/free-solid-svg-icons";
    import Button from "./Button.svelte";
    import Fa from "svelte-fa";
    import AddTransactionModal from "../AddTransactionModal.svelte";

    let { currentPage = $bindable('home'), showModal = $bindable(false) } = $props();

    let dateObj: Date = $state(new Date(Date.now()));
    
    let date: String = $state('');
    
    // let currentPage = $state('home');

    date = `${dateObj.getDate()}/${dateObj.getMonth() + 1}`;

</script>

<div class="fixed bottom-0 w-full bg-white p-0 flex justify-center z-40 shadow-nav shadow-blue-200">
    <div class="bg-white w-full flex flex-col justify-center rounded-lg shadow-xl shadow-blue-100">
        <div class="flex flex-row items-center justify-evenly w-full text-black bg-white">
            <Button icon={faBook} title={date} target="/" name="home" bind:currentPage onclick={() => currentPage === 'home'} />
            <Button icon={faChartPie} title="Stats" target="/stats" name="stats" bind:currentPage onclick={() => currentPage === 'stats'} />
            {#if localStorage.getItem('role') !== 'member'}
                <div class="bg-blue-900 rounded-xl flex justify-center items-center w-10 h-10 aspect-square active:scale-90 duration-100">
                    <button data-sveltekit-preload-data="tap" onclick={() => showModal = !showModal} class="flex flex-col items-center justify-center min-w-20 min-h-20 max-w-28 text-xl transition-transform active:scale-90 duration-100 text-white">
                        <Fa icon={faPlus} size="1x"/>
                    </button>
                </div>
            {/if}
            <Button icon={faClockRotateLeft} title="History" target="/history" name="history" bind:currentPage onclick={() => currentPage === 'history'} />
            <Button icon={faGear} title="Settings" target="/settings" name="settings" bind:currentPage onclick={() => currentPage === 'settings'}
            />
        </div>
    </div>
</div>
{#if showModal}
    <AddTransactionModal bind:showModal />
{/if}
