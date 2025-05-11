import { createQwikCity, type PlatformNode } from '@builder.io/qwik-city/middleware/node';
import { manifest } from '@qwik-client-manifest';
import qwikCityPlan from '@qwik-city-plan';
import render from './entry.ssr';

declare global {
  type QwikCityPlatform = PlatformNode;
}

// Create the Qwik City node middleware
export default createQwikCity({ render, qwikCityPlan, manifest });