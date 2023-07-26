// Send 트레잇은 타입의 소유권이 스레드 사이에서 이전될 수 있음을 의미한다.
// Rc<T> 는 Send 트레잇을 구현하지 않았다.
// Sync 트레잇은 타입이 여러 스레드로부터 안전하게 참조 가능함을 의미한다.
// 하지만, Send 와 Sync 를 직접 구현하는 것은 안전하지 않기 때문에 조심해야 한다.
pub fn sync_and_send_example() {

}