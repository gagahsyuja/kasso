<script lang="ts">
    import Fa from "svelte-fa";
    import Database from "@tauri-apps/plugin-sql";
    import { faUser, faLock } from "@fortawesome/free-solid-svg-icons";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { scale } from "svelte/transition";
    import { sha256, type Message } from "js-sha256";

    let username = $state();
    let password = $state();
    let wrongCredential = $state(false);

    const checkPassword = async () => {

        let hashedPassword = sha256(password as Message);

        const db = await Database.load("sqlite:database.db");

        let result: any = await db.select("SELECT * FROM users\
            WHERE username = $1 AND password = $2 LIMIT 1",
            [username, hashedPassword]);

        if (result.length) {
            localStorage.setItem('username', result[0].username)
            localStorage.setItem('role', result[0].role)
            localStorage.setItem('notify', result[0].notify)
            goto('/', { replaceState: true })
            wrongCredential = false;
        } else {
            wrongCredential = true;
        }
    };

    onMount(() => {
        if (localStorage.getItem('username')) goto('/', { replaceState: true })
    })
</script>

<div class="flex flex-col justify-center items-center pb-56">
    <h1 class="text-6xl font-extrabold pt-32 pb-32 text-blue-900">
        KASSO
    </h1>
    <div class="flex flex-col w-full justify-center items-center px-8 space-y-6">
        <!-- {#if wrongCredential} -->
        {#key wrongCredential}
            <div class="{wrongCredential ? "opacity-100" : "opacity-0" } rounded-xl w-full text-center
                font-bold px-6 py-2 bg-red-500 text-white"
                in:scale|global={{ duration: 100 }}
            >
                Wrong username or password!
            </div>
        {/key}
        <!-- {/if} -->
        <div class="bg-white flex flex-col bg-red-500/0 px-8 my-0 w-full rounded-3xl shadow-xl shadow-blue-100">
            <label class="pt-2 text-gray-800" for="email">Username</label>
            <div class="flex flex-row justify-center items-center pb-2">
                <Fa icon={faUser} size="1.3x" class="" />
                <input class="mx-2 border-0 w-full focus:ring-0 font-bold text-black text-xl" type="text" id="username" bind:value={username} />
            </div>
        </div>
        <div class="bg-white flex flex-col bg-red-500/0 px-8 my-0 w-full rounded-3xl shadow-xl shadow-blue-100">
            <label class="pt-2 text-gray-800" for="password">Password</label>
            <div class="flex flex-row justify-center items-center pb-2">
                <Fa icon={faLock} size="1.3x" class="" />
                <input class="mx-2 border-0 w-full focus:ring-0 font-bold text-black text-xl" type="password" id="password" bind:value={password} />
            </div>
        </div>
    </div>
    <div class="w-full px-8 py-7">
        <button onclick="{checkPassword}" class="bg-blue-900 text-white font-semibold p-4 w-full rounded-[30px]">Login</button>
    </div>
    <a href="/register">Create Account</a>
</div>

<style>
    :global(html) {
        background-color: rgb(239 246 255);
    }
</style>
