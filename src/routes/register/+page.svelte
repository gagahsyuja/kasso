<script lang="ts">
    import Fa from "svelte-fa";
    import Database from "@tauri-apps/plugin-sql";
    import { faUser, faLock, faKey } from "@fortawesome/free-solid-svg-icons";
    import { scale } from "svelte/transition";
    import { goto } from "$app/navigation";
    import { sha256 } from "js-sha256";

    let username = $state('');
    let password = $state('');
    let confirmPassword = $state('');
    let code = $state('');
    let message = $state('');
    let role = $state('member');

    const checkPassword = () => {

        if (password !== confirmPassword) {
            message = 'Password does not match!';
            return false;
        }

        return true;
    };

    const checkUsername = async () => {

        const db = await Database.load("sqlite:database.db");

        let result: any = await db.select("SELECT * FROM users WHERE username = $1", [username]);

        if (result.length) {
            message = 'Username already exist!';
            return false;
        }

        return true;
    };

    const checkCode = () => {
        if (code === '999') role = 'treasurer';
    };

    const handleRegister = async () => {

        if (checkPassword() && await checkUsername()) {

            let hashedPassword = sha256(password);

            checkCode();

            const db = await Database.load("sqlite:database.db");

            await db.execute(
                "INSERT INTO users (role, username, password, notify) VALUES ($1, $2, $3, $4)", 
                [ role, username, hashedPassword, 500000 ]
            );

            message = '';
            localStorage.setItem('username', username)
            localStorage.setItem('role', role)
            goto('/', { replaceState: true });
        }
    };
</script>

<div class="flex flex-col justify-center items-center pb-56">
    <h1 class="text-6xl font-extrabold pt-10 pb-10 text-blue-900">
        KASSO
    </h1>
    <div class="flex flex-col w-full justify-center items-center px-8 space-y-6">
        {#key message}
            <div class="{message !== '' ? "opacity-100" : "opacity-0" } rounded-xl w-full text-center
                font-bold px-6 py-2 bg-red-500 text-white"
                in:scale|global={{ duration: 100 }}
            >
                {message}
            </div>
        {/key}
        <div class="bg-white flex flex-col bg-red-500/0 px-6 my-0 w-full rounded-3xl shadow-xl shadow-blue-100">
            <label class="pt-2 text-gray-800" for="email">Username</label>
            <div class="flex flex-row justify-center items-center pb-2">
                <Fa icon={faUser} size="1.2x" class="" />
                <input class="mx-2 border-0 w-full focus:ring-0 font-bold text-black text-xl" type="text" id="username" bind:value={username} />
            </div>
        </div>
        <div class="bg-white flex flex-col bg-red-500/0 px-6 my-0 w-full rounded-3xl shadow-xl shadow-blue-100">
            <label class="pt-2 text-gray-800" for="password">Password</label>
            <div class="flex flex-row justify-center items-center pb-2">
                <Fa icon={faLock} size="1.2x" class="" />
                <input class="mx-2 border-0 w-full focus:ring-0 font-bold text-black text-xl" type="password" id="password" bind:value={password} />
            </div>
        </div>
        <div class="bg-white flex flex-col bg-red-500/0 px-6 my-0 w-full rounded-3xl shadow-xl shadow-blue-100">
            <label class="pt-2 text-gray-800" for="confirmPassword">Confirm Password</label>
            <div class="flex flex-row justify-center items-center pb-2">
                <Fa icon={faLock} size="1.2x" class="" />
                <input class="mx-2 border-0 w-full focus:ring-0 font-bold text-black text-xl" type="password" id="confirmPassword" bind:value={confirmPassword} />
            </div>
        </div>
        <div class="bg-white flex flex-col bg-red-500/0 px-6 my-0 w-full rounded-3xl shadow-xl shadow-blue-100">
            <label class="pt-2 text-gray-800" for="code">Secret Code (Optional)</label>
            <div class="flex flex-row justify-center items-center pb-2">
                <Fa icon={faKey} size="1.2x" class="" />
                <input class="mx-2 border-0 w-full focus:ring-0 font-bold text-black text-xl" type="text" id="code" bind:value={code} />
            </div>
        </div>
    </div>
    <div class="w-full px-8 py-7">
        <button onclick="{handleRegister}" class="bg-blue-900 text-white font-semibold p-4 w-full rounded-[30px]">Register</button>
    </div>
    <a href="/login">Login</a>
</div>

<style>
    :global(html) {
        background-color: rgb(239 246 255);
    }
</style>
