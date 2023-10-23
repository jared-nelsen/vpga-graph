(ns vpga-graph.pin)

(defrecord Pin [id state neighbor-pin-ids])

(defn generate
  "Generates an unconnected pin"
  []
  (Pin. "" 0 []))

(defn reset
  "Resets a pin's state to off"
  [pin]
  (assoc pin :state 0))

(defn neighbors
  "Returns the neighbor pin-ids from the given Pin"
  [pin]
  (:neighbor-pin-ids pin))

(defn connect-to
  "Connects the given pin to another given pin"
  [pin other-pin]
  (let [pin-neighbors (:neighbor-pin-ids pin)
        updated-neighbors (conj pin-neighbors (:id other-pin))]
    (assoc pin :neighbor-pin-ids updated-neighbors)))