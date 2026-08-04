import React from 'react';
import Link from '@docusaurus/Link';

export default function Homepage() {
  return React.createElement('main', null,
    React.createElement('h1', null, 'Archr'),
    React.createElement('p', null, 'Headless ArchiMate 3.2 engine — validate, manipulate, export architecture models'),
    React.createElement(Link, { to: '/introduction' }, 'Read the docs →')
  );
}
