1)

sender
program_id
message -> send~to=Andrey;amount=100


2) 

sender
amount
receiver/program
message

##############################
blockchain
- Blocks
  - tx
  - tx1

+) ---

-) устарела
   Более медленно

##############################


blockchain
- Blocks
   - History
    - tx
    - tx1

+) Чёрная магия Solana

-) --- 

##############################

History
 - tx
 - tx1

+) Скорость
   Протота реализации
   Чёрная магия Solana

-) Невозможность синхронизироваться при высоком TPS (T/s()Транзакции в секунду)

Трилема:
1) Безопасность
2) Скорость(масштабируемость)
3) Децентрализация

##############################


DAG (Directed Acyclic Graph)
???

+) ---

-) --- 


##############################


Proof of Work:  1/10 Безопасность
Proof of Stake: 7/10 Скорость Безопасность
Proof of History: (Все 3)
Proof of Authority: Скорость++ Безопасность !!! 90% централизации !!!

##############################

Parallel Runtime

will_touch -> String[]

sc1
sc2
sc3

execute wrapper
    - executor1(sc1)
        ...running code
        invoke!
    - executor2(sc2)
        ...running code

# First: an executor should spawn a new executor! Also, the child executor will have the same permissions as the parent one!
# Second: the LayerRuntime is split into PreRutime and RuntimeRuntime. All transacrtions are stored in the PreRuntime but then moved in to the RuntimeRuntime where there are actually executed. When one program starts another in is directly invoked in RuntimeRuntime beacose the first rule will fix any issues with this.

Итог:

1) Цель: создать бесплатную бекенд платформу
2) Механизм консенсуса: Proof of Authority + Proof of History
3) Смарт Контракты - ЕСТЬ!
4) ЯП: HashScript
5) ЯПЫ для клиента: JavaScript (возможно в далёком будующем Python)
6) Валидаторы должны иметь белый IP
7) Всё монироиться через Uptime Kuma
