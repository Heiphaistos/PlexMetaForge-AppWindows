import { test } from 'node:test';
import assert from 'node:assert/strict';
import { progressPercent } from './updateProgress.ts';

test('progressPercent', () => {
  assert.equal(progressPercent(50, 200), 25);
  assert.equal(progressPercent(300, 200), 100);
  assert.equal(progressPercent(10, 0), null);
});
