import { render } from '@builder.io/qwik';
import Root from './root';

export default function (opts) {
  return render(document, <Root />, opts);
} 