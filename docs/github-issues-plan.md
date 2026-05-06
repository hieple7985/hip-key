# hip-key Roadmap

## Tổng quan chiến lược

**Mô hình:** B2C Freemium
- **Free:** macOS Telex IME + dictionary + basic learning
- **Paid ($2.99-4.99/mo):** AI local correction (privacy-first, offline)
- **Expansion:** Cross-device sync, iOS, other languages

**Target người dùng:**
- Vietnamese diaspora ở Mỹ/EU/AU (beachhead market)
- Writers, developers, content creators Việt Nam cần gõ tiếng Việt nhiều

---

## Feature Analysis: Cái nào đủ lý do để chuyển + trả tiền

### ✅ ĐỦ LÝ DO CHUYỂN (Switching Motivation)

#### Reason #1: macOS Telex (HIGHEST PRIORITY)
**Pain point cụ thể:** macOS không có Telex thật sự. Chỉ có VNI hoặc dead-key. Người Việt ở Mỹ/EU đã quen Telex từ Windows.

**Sản phẩm:** macOS IME với Telex đầy đủ như Unikey, UI hiện đại.

**Lý do free users sẽ switch:** Đây là vấn đề thực sự họ gặp hàng ngày. Không có giải pháp nào tốt trên macOS.

**Đã làm xong:** Core Telex/VNI engine ✅

---

#### Reason #2: AI Local Correction
**Pain point:** Viết sai chính tả tiếng Việt, không có tool nào sửa được local (Grammarly không support tiếng Việt).

**Sản phẩm:** Local AI model (mistral.cpp) chạy offline, privacy-first, như Grammarly cho tiếng Việt.

**Lý do users sẽ trả tiền:** Tạo ra value rõ ràng mỗi ngày cho writer/content creator.

**Chưa làm** → Phase 1

---

#### Reason #3: Modern UI (Nice to have, không đủ để trả tiền)
**Sản phẩm:** UI đẹp như app hiện đại, không phải như Unikey 1990s.

**Giá trị:** Tạo ấn tượng đầu tiên tốt, nhưng không đủ lý do switch nếu không có Reason #1 hoặc #2.

**Làm cùng với macOS adapter**

---

#### Reason #4: Never Crash / Reliable
**Pain point:** Unikey chết bất chợt, mất buffer khi gõ.

**Sản phẩm:** IME không bao giờ crash, buffer được backup.

**Lý do nhỏ nhưng có:** Đặc biệt quan trọng với developers/writers.

---

### ❌ KHÔNG ĐỦ LÝ DO CHUYỂN (Already have in free competitors)

| Feature | Free Competitors | Lý do không đủ |
|---|---|---|
| Dictionary 50k+ từ | Unikey, EVkey có đủ | Không tạo differentiation |
| Macro expansion | Unikey có | Người dùng không dùng |
| Config system | Unikey có | Nice to have, không switch reason |
| Multiple input methods | Tất cả đều có | Table stakes |
| Frequency learning | EVkey có | Không đủ justify switching cost |

---

## Trạng thái hiện tại

### ✅ ĐÃ LÀM XONG

| Component | Status | Notes |
|---|---|---|
| Core engine (keystroke, buffer, langpack) | ✅ Done | Language-agnostic |
| Vietnamese Telex input | ✅ Done | aw→ă, aa→â, tone marks đầy đủ |
| Vietnamese VNI input | ✅ Done | a8→ă, a6→â, tone marks 1-5 |
| Dictionary (Trie + 300+ words) | ✅ Done | Expandable |
| Frequency learning system | ✅ Done | Local storage |
| C FFI API | ✅ Done | C header generated |
| Config system | ✅ Done | TOML-based |
| Macro expansion | ✅ Done | Default abbreviations |
| Benchmarks | ✅ Done | Criterion.rs setup |
| Security hardening | ✅ Done | FFI null checks, alloc guards |
| macOS adapter skeleton | 🔨 WIP | Swift + IMKKit, chưa complete |

### ❌ CHƯA LÀM (Prioritized)

| Feature | Priority | Notes |
|---|---|---|
| macOS Telex IME hoàn chỉnh | **P0 - NOW** | Beachhead product |
| AI local correction | **P1 - Next** | Conversion driver |
| macOS build + install | **P0 - NOW** | Phải build được mới test được |
| macOS App Store distribution | P1 | Revenue path |
| iOS app | P2 | Expand ecosystem |
| Cross-device sync | P2 | Retention, lock-in |
| Polish language pack | P3 | Expansion vector |

---

## Roadmap by Phase

### Phase P0: macOS v1.0 (NOW - 2-3 tuần)

**Mục tiêu:** Release macOS Telex IME miễn phí, acquire users

**Tasks:**
- [ ] Complete macOS IMKInputController implementation
- [ ] Fix Swift FFI bridging (libhip_key_ffi.a)
- [ ] Build .app bundle đúng format
- [ ] Test Telex: `aws` → `ắ`, `dd` → `đ`, `chaof` → `chào`
- [ ] Test backspace, arrow keys, escape
- [ ] Test candidate selection (số 1-9)
- [ ] Create AppIcon (simple icon design)
- [ ] Write README cho macOS installation

**Deliverable:** `.app` file + installation guide, users có thể install thủ công

**Revenue model:** Free (acquisition phase)

---

### Phase P1: AI Local Correction (1-2 tháng)

**Mục tiêu:** Paid tier với AI correction, tạo revenue

**Tasks:**
- [ ] Integrate mistral.cpp (hoặc llama.cpp) vào core
- [ ] Train/fine-tune model cho Vietnamese spelling correction
- [ ] Buffer correction suggestions (highlight misspelled words)
- [ ] Accept/reject correction UI
- [ ] Offline-first (không cần internet)
- [ ] Performance: < 50ms per correction

**Revenue model:**
```
Free: Basic Telex + Dictionary
Paid ($2.99/mo): AI correction
Paid+ ($4.99/mo): AI + cloud backup + themes
```

**Deliverable:** Paid upgrade feature

---

### Phase P2: Distribution & Growth (1-3 tháng)

**Mục tiêu:** Scale user base

**Tasks:**
- [ ] macOS App Store submission
- [ ] Apple Developer account ($99/năm)
- [ ] App Store listing optimization (keywords, screenshots)
- [ ] Website: landing page, download
- [ ] Viral: "tell a friend" referral program
- [ ] Community: Vietnamese subreddits, forums

**Revenue model:**
- App Store subscription (Apple takes 15-30%)
- Direct website sales (higher margin)

---

### Phase P3: iOS + Cross-device (3-6 tháng)

**Mục tiêu:** Lock-in users vào ecosystem

**Tasks:**
- [ ] iOS keyboard extension (InputMethodKit trên iOS)
- [ ] iCloud sync cho learning data
- [ ] Sync preferences across devices
- [ ] iOS App Store launch

**Revenue model:**
- Cross-device sync là feature của Paid+ tier

---

### Phase P4: Polish Language Pack (6-12 tháng)

**Mục tiêu:** Mở rộng ra thị trường mới

**Tasks:**
- [ ] Polish language pack (35M speakers)
- [ ] Czech/Slovak/Hungarian (15M speakers)
- [ ] Romanian (25M speakers)
- [ ] Generalize architecture cho easy lang pack creation

**Revenue model:** Same freemium model, new markets

---

## Chi tiết kỹ thuật: AI Local Correction

### Tại sao local (on-device) thay vì cloud?

| | Local AI | Cloud AI |
|---|---|---|
| Privacy | ✅ 100% private | ❌ Keystrokes leave device |
| Cost | ✅ Near zero marginal cost | ❌ API costs per request |
| Latency | ✅ < 50ms | ❌ 200-500ms network |
| Offline | ✅ Works without internet | ❌ Fails offline |
| Accuracy | ✅ Fine-tuned cho VN | ✅ Better models possible |

### Technology Stack

- **Model:** mistral.cpp hoặc tinyllama quantized (< 100MB)
- **Fine-tuning data:** Vietnamese Wikipedia, news articles, common typos
- **Inference:** On-device, < 50ms latency
- **Storage:** Compressed model bundled với app

### Correction Flow

```
User types: "toi di hoc"
    ↓
Local AI detects: "hoc" không trong dictionary
    ↓
Suggests: "học" (confidence 0.85)
    ↓
User accepts (Tab/Enter) or ignores
    ↓
If accepted: Update learning frequency
```

### Pricing cho AI correction

- **$2.99/tháng** hoặc **$24.99/năm** (giảm ~30%)

**Psychological pricing:** Để giá $2.99 thay vì $3.00 — nghe rẻ hơn mà psychological effect đủ.

---

## Metrics & Goals

### Năm 1

| Metric | Goal |
|---|---|
| macOS downloads | 50,000 |
| Monthly active users | 10,000 |
| Paid subscribers | 500-1,000 |
| Monthly revenue | $1,500-3,000 |
| App Store rating | 4.5+ stars |

### Năm 2

| Metric | Goal |
|---|---|
| macOS + iOS users | 200,000 |
| Paid subscribers | 5,000+ |
| MRR | $15,000 |
| ARR | $180,000 |

---

## Risks & Mitigations

| Risk | Likelihood | Mitigation |
|---|---|---|
| Apple rejects macOS IME app | Medium | Follow guidelines carefully, use proper entitlements |
| Users don't convert to paid | High | Focus on AI correction quality, show value in free tier |
| Large company copies (Google, Apple) | Low-Medium | They won't build for small VN market specifically |
| AI model too slow on old Macs | Medium | Use small quantized model, graceful degradation |

---

## Order of Implementation (Revised)

```
NOW:       macOS v1.0 (P0)
Month 1-2: AI correction integration (P1)
Month 2-3: App Store submission (P2)
Month 3-6: iOS app (P3)
Month 6-12: Polish language pack (P4)
```

---

## Labels cho GitHub Issues

- `p0` - Must have (macOS Telex)
- `p1` - Should have (AI correction)
- `p2` - Nice to have (iOS, sync)
- `enhancement` - Feature request
- `bug` - Bug report
- `platform:macos` / `platform:ios` / `platform:windows`
- `ai` - AI-related work
- `good first issue` - Beginner-friendly