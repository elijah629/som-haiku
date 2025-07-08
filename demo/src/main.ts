import { filter_is_haiku, into_haiku } from "./pkg/som_haiku";

const $fileUpload = document.getElementById("file-upload") as HTMLInputElement;
const $filterButton = document.getElementById("filter-button") as HTMLButtonElement;
const $filterOutput = document.getElementById("filter-output") as HTMLTextAreaElement;
const $filterCount = document.getElementById("filter-count") as HTMLSpanElement;

const $idButton = document.getElementById("id-button") as HTMLButtonElement;
const $idIO = document.getElementById("id-io") as HTMLTextAreaElement;

const decoder = new TextDecoder();

let file: Uint8Array<ArrayBufferLike> | undefined = undefined;

$fileUpload.addEventListener("change", async () => {
  file = await $fileUpload.files?.[0].bytes();
});

$filterButton.addEventListener("click", async () => {
  if (file === undefined) file = await fetch("/hackclub").then(x => x.bytes());

  if (file) {
   $filterOutput.value = decoder.decode(filter_is_haiku(file)).trimEnd();
   $filterCount.innerText = "Count: " + $filterOutput.value.split("\n").length.toString();
  }
});

$idButton.addEventListener("click", async () => {
  const id = $idIO.value;

  $idIO.value = into_haiku(id);
});
