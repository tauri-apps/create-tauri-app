import {
  createQwikCity,
  type PlatformNode
} from '@builder.io/qwik-city/middleware/node';
import qwikCityPlan from '@qwik-city-plan';
import { manifest } from '@qwik-client-manifest';
import render from './entry.ssr';
import { fileURLToPath } from 'node:url';
import { join } from 'node:path';

declare global {
  interface QwikCityPlatform extends PlatformNode {}
}

// Determine the production build directory
const distDir = join(fileURLToPath(import.meta.url), '..', '..');

// Create the Qwik City node middleware
export default createQwikCity({ render, qwikCityPlan, manifest, distDir }); 