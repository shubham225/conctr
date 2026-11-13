// Load colors/tokens.css globally. Colors, spacing, radius and shadows all read CSS variables,
// so light and dark mode switch without extra Tailwind classes.
module.exports = {
  darkMode: ["selector", '[data-theme="dark"]'],
  theme: {
    extend: {
      colors: { cn: {
      "bg": "var(--cn-bg)",
      "surface": "var(--cn-surface)",
      "raised": "var(--cn-raised)",
      "sunken": "var(--cn-sunken)",
      "border": "var(--cn-border)",
      "border-strong": "var(--cn-border-strong)",
      "text": "var(--cn-text)",
      "text-muted": "var(--cn-text-muted)",
      "text-subtle": "var(--cn-text-subtle)",
      "primary": "var(--cn-primary)",
      "primary-hover": "var(--cn-primary-hover)",
      "primary-soft": "var(--cn-primary-soft)",
      "on-primary": "var(--cn-on-primary)",
      "teal": "var(--cn-teal)",
      "teal-text": "var(--cn-teal-text)",
      "teal-soft": "var(--cn-teal-soft)",
      "coral": "var(--cn-coral)",
      "coral-text": "var(--cn-coral-text)",
      "coral-soft": "var(--cn-coral-soft)",
      "amber": "var(--cn-amber)",
      "amber-text": "var(--cn-amber-text)",
      "amber-soft": "var(--cn-amber-soft)",
      "success": "var(--cn-success)",
      "success-soft": "var(--cn-success-soft)",
      "warning": "var(--cn-warning)",
      "warning-soft": "var(--cn-warning-soft)",
      "danger": "var(--cn-danger)",
      "danger-soft": "var(--cn-danger-soft)",
      "on-danger": "var(--cn-on-danger)",
      "info": "var(--cn-info)",
      "info-soft": "var(--cn-info-soft)",
      "focus": "var(--cn-focus)"
} },
      fontFamily: { display: ["var(--cn-font-display)"], sans: ["var(--cn-font-ui)"], mono: ["var(--cn-font-mono)"] },
      fontSize: {
      "display": [
            "var(--cn-fs-display)",
            {
                  "lineHeight": "var(--cn-lh-display)",
                  "letterSpacing": "var(--cn-ls-display)"
            }
      ],
      "h1": [
            "var(--cn-fs-h1)",
            {
                  "lineHeight": "var(--cn-lh-h1)",
                  "letterSpacing": "var(--cn-ls-h1)"
            }
      ],
      "h2": [
            "var(--cn-fs-h2)",
            {
                  "lineHeight": "var(--cn-lh-h2)",
                  "letterSpacing": "var(--cn-ls-h2)"
            }
      ],
      "h3": [
            "var(--cn-fs-h3)",
            {
                  "lineHeight": "var(--cn-lh-h3)",
                  "letterSpacing": "var(--cn-ls-h3)"
            }
      ],
      "h4": [
            "var(--cn-fs-h4)",
            {
                  "lineHeight": "var(--cn-lh-h4)",
                  "letterSpacing": "var(--cn-ls-h4)"
            }
      ],
      "body-lg": [
            "var(--cn-fs-body-lg)",
            {
                  "lineHeight": "var(--cn-lh-body-lg)",
                  "letterSpacing": "var(--cn-ls-body-lg)"
            }
      ],
      "body": [
            "var(--cn-fs-body)",
            {
                  "lineHeight": "var(--cn-lh-body)",
                  "letterSpacing": "var(--cn-ls-body)"
            }
      ],
      "small": [
            "var(--cn-fs-small)",
            {
                  "lineHeight": "var(--cn-lh-small)",
                  "letterSpacing": "var(--cn-ls-small)"
            }
      ],
      "caption": [
            "var(--cn-fs-caption)",
            {
                  "lineHeight": "var(--cn-lh-caption)",
                  "letterSpacing": "var(--cn-ls-caption)"
            }
      ],
      "code": [
            "var(--cn-fs-code)",
            {
                  "lineHeight": "var(--cn-lh-code)",
                  "letterSpacing": "var(--cn-ls-code)"
            }
      ]
},
      spacing: {
      "0": "var(--cn-space-0)",
      "1": "var(--cn-space-1)",
      "2": "var(--cn-space-2)",
      "3": "var(--cn-space-3)",
      "4": "var(--cn-space-4)",
      "5": "var(--cn-space-5)",
      "6": "var(--cn-space-6)",
      "8": "var(--cn-space-8)",
      "10": "var(--cn-space-10)",
      "12": "var(--cn-space-12)",
      "16": "var(--cn-space-16)",
      "20": "var(--cn-space-20)",
      "24": "var(--cn-space-24)"
},
      borderRadius: {
      "xs": "var(--cn-radius-xs)",
      "sm": "var(--cn-radius-sm)",
      "md": "var(--cn-radius-md)",
      "lg": "var(--cn-radius-lg)",
      "xl": "var(--cn-radius-xl)",
      "pill": "var(--cn-radius-pill)"
},
      boxShadow: {
      "1": "var(--cn-shadow-1)",
      "2": "var(--cn-shadow-2)",
      "3": "var(--cn-shadow-3)",
      "4": "var(--cn-shadow-4)"
},
      zIndex: {
      "base": "0",
      "sticky": "100",
      "dropdown": "200",
      "overlay": "300",
      "modal": "400",
      "toast": "500",
      "tooltip": "600"
},
      screens: {
      "sm": "640px",
      "md": "768px",
      "lg": "1024px",
      "xl": "1280px",
      "2xl": "1536px"
},
    },
  },
};
