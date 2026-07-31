<template>
  <main class="messenger-layout">
    <aside class="messenger-sidebar" aria-label="Messenger sidebar">
      <header class="messenger-sidebar__header">
        <slot name="sidebar-header">
          <div class="layout-placeholder">Sidebar header</div>
        </slot>
      </header>

      <div class="messenger-sidebar__content">
        <slot name="sidebar">
          <div class="layout-placeholder layout-placeholder--fill">Sidebar</div>
        </slot>
      </div>
    </aside>

    <section class="messenger-main" aria-label="Messenger main area">
      <header class="messenger-topbar">
        <slot name="topbar">
          <div class="layout-placeholder">Top bar</div>
        </slot>
      </header>

      <section class="messenger-content" aria-label="Messenger content">
        <slot>
          <div class="layout-placeholder layout-placeholder--fill">
            Main content
          </div>
        </slot>
      </section>
    </section>
  </main>
</template>
