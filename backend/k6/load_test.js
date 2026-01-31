import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  stages: [
    { duration: '30s', target: 50 },    // Ramp-up: 0 to 50 users
    { duration: '1m', target: 200 },   // Stress: Steady at 200 users
    { duration: '30s', target: 0 },     // Ramp-down
  ],
  thresholds: {
    // 95% of requests should be below 50ms for high-concurrency validation
    http_req_duration: ['p(95)<50'], 
    http_req_failed: ['rate<0.01'],   // Error rate should be less than 1%
  },
};

export default function () {
  const url = 'http://localhost:8000/bets';
  
  const payload = JSON.stringify({
    user_id: "550e8400-e29b-41d4-a716-446655440000",
    match_id: "123e4567-e89b-12d3-a456-426614174000",
    amount: 10.50,
    odds: 1.85,
  });

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  const res = http.post(url, payload, params);
  
  check(res, {
    'status is 200': (r) => r.status === 200,
    'latency is low': (r) => r.timings.duration < 20,
  });
  
  // Minimal sleep to simulate high concurrency throughput
  sleep(0.1);
}
