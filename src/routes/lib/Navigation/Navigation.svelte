<script lang="ts">
    import Fa from "svelte-fa";
    import { faBook, faChartPie, faGear } from "@fortawesome/free-solid-svg-icons";
    import Button from "./Button.svelte";

    let date: any = new Date(Date.now());
    date = `${date.getDate()}/${date.getMonth() + 1}`;
</script>

<div class="bg-red-500 absolute bottom-0 left-0 h-20 inset-x-0 mx-auto flex flex-col justify-center">
    <div class="flex flex-row items-center justify-evenly w-full">
        <Button icon={faBook} title={date} />
        <Button icon={faChartPie} title="Stats" />
        <Button icon={faGear} title="Settings" />
    </div>
</div>
