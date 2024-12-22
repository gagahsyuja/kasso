<script lang="ts">
    import Fa from "svelte-fa";
    import { faEnvelope, faLock } from "@fortawesome/free-solid-svg-icons";
</script>

<div class="flex flex-col justify-center items-center pb-56">
    <h1 class="text-8xl font-extrabold pt-32 pb-40 text-blue-900">
        KASSO
    </h1>
    <div class="flex flex-col w-full justify-center items-center px-8 space-y-6">
        <div class="bg-white flex flex-col bg-red-500/0 px-8 my-0 w-full rounded-3xl shadow-xl shadow-blue-100">
            <label class="pt-2 text-gray-800" for="email">Email Address</label>
            <div class="flex flex-row justify-center items-center pb-2">
                <Fa icon={faEnvelope} size="1.3x" class="" />
                <input class="mx-2 border-0 w-full focus:ring-0 font-bold text-black text-xl" type="email" id="email" />
            </div>
        </div>
        <div class="bg-white flex flex-col bg-red-500/0 px-8 my-0 w-full rounded-3xl shadow-xl shadow-blue-100">
            <label class="pt-2 text-gray-800" for="password">Password</label>
            <div class="flex flex-row justify-center items-center pb-2">
                <Fa icon={faLock} size="1.3x" class="" />
                <input class="mx-2 border-0 w-full focus:ring-0 font-bold text-black text-xl" type="password" id="password" />
            </div>
        </div>
    </div>
    <div class="w-full px-8 py-7">
        <button onclick="{window.location.href = "/"}" class="bg-blue-900 text-white font-semibold p-4 w-full rounded-[30px]">Login</button>
    </div>
    <a href="#">Create Account</a>
</div>

<style>
    :global(html) {
        background-color: rgb(239 246 255);
    }
</style>
