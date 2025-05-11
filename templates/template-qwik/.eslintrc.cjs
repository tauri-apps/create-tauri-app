module.exports = {
  root: true,
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:qwik/recommended',
  ],
  parserOptions: {
    ecmaVersion: 2021,
    sourceType: 'module',
    ecmaFeatures: {
      jsx: true,
    },
  },
  plugins: ['qwik'],
  rules: {
    'no-case-declarations': 'off',
    'no-console': 'off',
    'qwik/jsx-img': 'off',
    'qwik/loader-location': 'off',
    'qwik/valid-lexical-scope': 'warn',
  },
}; 