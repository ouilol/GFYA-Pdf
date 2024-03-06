<script lang="ts">
  import "../app.css";
</script>

<div class="navbar bg-neutral">
  <div class="flex-1">
    <a href="/" class="btn btn-ghost text-xl">GFYA-Pdf</a>
  </div>
  <div class="content-start">
    <ul class="menu menu-horizontal px-1">
        <li><a href="/split">PDF Split</a></li>
        <li><a href="/remove">PDF Remove</a></li>
        <li><a href="/insert">PDF Insert</a></li>
        <li><a href="/insert">PDF Merge</a></li>
        <li><a href="/watermark">PDF watermark</a></li>
    </ul>
  </div>
</div>

<slot />
