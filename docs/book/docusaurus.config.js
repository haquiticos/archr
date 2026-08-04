/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'archr',
  tagline: 'Headless ArchiMate 3.2 engine',
  url: 'https://haquiticos.github.io',
  baseUrl: '/archr/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  organizationName: 'haquiticos',
  projectName: 'archr',
  presets: [
    [
      'classic',
      {
        docs: {
          id: 'docs',
          path: 'docs',
          routeBasePath: '/',
          sidebarPath: require.resolve('./sidebars.js'),
        },
        blog: false,
      },
    ],
  ],
};
module.exports = config;
