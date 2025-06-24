import { version } from "./package.json";
import { defineConfig } from "vocs";

export default defineConfig({
  description:
    "DBX is a minimal, blazing-fast API proxy for edge and embedded systems. Expose Redis, Qdrant, and MDBX through a unified API layer.",
  rootDir: ".",
  sidebar: [
    {
      text: "Getting Started",
      items: [
        {
          text: "Introduction",
          link: "/docs",
        },
        {
          text: "Quick Start",
          link: "/docs/getting-started",
        },
        {
          text: "Installation",
          link: "/docs/installation",
        },
        {
          text: "Configuration",
          link: "/docs/configuration",
        },
      ],
    },
    {
      text: "API Reference",
      items: [
        {
          text: "REST API",
          items: [
            {
              text: "Authentication",
              link: "/docs/api/rest/authentication",
            },
            {
              text: "String Operations",
              link: "/docs/api/rest/string",
            },
            {
              text: "Hash Operations",
              link: "/docs/api/rest/hash",
            },
            {
              text: "Set Operations",
              link: "/docs/api/rest/set",
            },
            {
              text: "Admin Operations",
              link: "/docs/api/rest/admin",
            },
            {
              text: "Error Handling",
              link: "/docs/api/rest/errors",
            },
          ],
        },
        {
          text: "WebSocket API",
          items: [
            {
              text: "Connection Setup",
              link: "/docs/api/websocket/connection",
            },
            {
              text: "String Operations",
              link: "/docs/api/websocket/string",
            },
            {
              text: "Hash Operations",
              link: "/docs/api/websocket/hash",
            },
            {
              text: "Set Operations",
              link: "/docs/api/websocket/set",
            },
            {
              text: "Admin Operations",
              link: "/docs/api/websocket/admin",
            },
          ],
        },
      ],
    },
    {
      text: "SDK Documentation",
      items: [
        {
          text: "TypeScript SDK",
          items: [
            {
              text: "Installation",
              link: "/docs/sdk/typescript/installation",
            },
            {
              text: "String Client",
              link: "/docs/sdk/typescript/string",
            },
            {
              text: "Hash Client",
              link: "/docs/sdk/typescript/hash",
            },
            {
              text: "Set Client",
              link: "/docs/sdk/typescript/set",
            },
            {
              text: "Admin Client",
              link: "/docs/sdk/typescript/admin",
            },
            {
              text: "WebSocket Client",
              link: "/docs/sdk/typescript/websocket",
            },
          ],
        },
      ],
    },
    {
      text: "Deployment & Operations",
      items: [
        {
          text: "Docker Deployment",
          link: "/docs/deployment/docker",
        },
        {
          text: "Kubernetes Deployment",
          link: "/docs/deployment/kubernetes",
        },
        {
          text: "Cloud Deployment",
          link: "/docs/deployment/cloud",
        },
        {
          text: "Edge Deployment",
          link: "/docs/deployment/edge",
        },
        {
          text: "Monitoring",
          link: "/docs/deployment/monitoring",
        },
        {
          text: "Troubleshooting",
          link: "/docs/deployment/troubleshooting",
        },
      ],
    },
    {
      text: "Advanced Topics",
      items: [
        {
          text: "Use Cases",
          link: "/docs/advanced/use-cases",
        },
        {
          text: "Performance",
          link: "/docs/advanced/performance",
        },
        {
          text: "Security",
          link: "/docs/advanced/security",
        },
        {
          text: "Architecture",
          link: "/docs/advanced/architecture",
        },
      ],
    },
    {
      text: "Development",
      items: [
        {
          text: "Development Guide",
          link: "/docs/development/guide",
        },
        {
          text: "Contributing",
          link: "/docs/development/contributing",
        },
        {
          text: "Changelog",
          link: "/docs/development/changelog",
        },
      ],
    },
  ],
  socials: [
    {
      icon: "github",
      link: "https://github.com/effortlesslabs/dbx",
    },
    {
      icon: "x",
      link: "https://x.com/effortlesslabs",
    },
  ],
  title: "DBX",
  topNav: [
    { text: "Guide & API", link: "/docs" },
    { text: "Blog", link: "/blog" },
    {
      text: version,
      items: [
        {
          text: "Changelog",
          link: "/docs/development/changelog",
        },
        {
          text: "Contributing",
          link: "/docs/development/contributing",
        },
      ],
    },
  ],
});
