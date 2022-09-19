(ns app.core-test
  (:require
   [app.core :as sut]
   [cljs.test :as t :include-macros true]))


(t/deftest square-test
  (t/testing "dummy test"
    (t/is (= 4 (sut/square 2)))))
