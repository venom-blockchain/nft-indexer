# NFT Indexer


В тестнет задеплоил новые контракты, с фиксами.
✔️ AuctionRootTip3 owner … 0:e06217c6ed5ec4ad90cf11af7cc3f5934ce68f146e7839454eb0712596cf6066
AuctionRootTip3: 0:9b10e4ab6be3ad33ce621d2c7aec866bdf81983e7e2ce660423fae9b29f0ca65

✔️ FactoryDirectBuy owner … 0:e06217c6ed5ec4ad90cf11af7cc3f5934ce68f146e7839454eb0712596cf6066
FactoryDirectBuy: 0:e7df75e4a0dafe74179ae8c588d02f98283d4fe73a2f08e0da9e6d9184bbe2ee

✔️ FactoryDirectSell owner … 0:e06217c6ed5ec4ad90cf11af7cc3f5934ce68f146e7839454eb0712596cf6066
FactoryDirectSell: 0:0ff6c5cc1dc1888b8d74a5ec13d652815e8f25043c22a2d8895c6d4b4d1dbe52

# Требования к АПИ

Helen BF, [11.08.2022 00:41]
ручки для маркетплейса:
История цены по нфт.  В ответе: дата+время  - цена в долларах по крусу на день продажи. Часовые  и дневные.

Детали нфт. В ответе: название, изображение, описание, овнер, адрес коллекции, название коллекции, атрибуты нфт, текущая цена и токен (если есть действующий аукцион - максимальный бид, на продаже-цена продажи), текущая цена в $ по текущему курсу, последняя цена (цена последней сделки) 

Детали коллекции. В ответе: изображения коллекции, название,  овнер, описание коллекции, самая низкая цена нфт в коллекции, количество овнеров нфт в коллекций, общая сумма внутри коллекци по нфт, дата создания.

Список коллекций. Фильтр: по овнеру. В ответе: изображения коллекции, название,  овнер, количество нфт в коллекции, самая низкая цена нфт в коллекции, количество овнеров нфт в коллекций. Сортирвока по дате создания. (Еще как вариант сортировка по среднимузначению  активностей в час/день)

Список нфт. Фильтры: по коллекции/нескольким коллекциям, по овнеру/овнерам,  по стоимости (от/до, токен/токены), статус нфт (на продаже, активный аукцион, не продаются и не на аукционе). В ответе: изображение, название нфт,  название коллекции, адрес коллекци, цена и токен (если нфт на аукционе - цена последнего бида, если на продаже цена продажи) , максимальный офер в $ по текущему курсу, если нфт на аукционе то стартовая цена и токен, дата начала и дата конца аукциона, адрес контракта аукциона,  если на продаже то цена и токен продажи, дата начала и дата конца продажи(если есть).

Список активных аукционов. Фильтры: по коллекции/ях, по овнеру/ам, по токену/ам, все. Сортировка по количсетво ставок/ среднее количество ставок в час/день. Сортировка по дате старта.  В ответе: изображение, название нфт,  название коллекции, адрес коллекци, токен и цена последнего бида, цена старта и токен, адата начала, дата конца аукциона.

Список  активных оферов по нфт. Фильтр: по токену.  В ответе:  стоимость офера (сумма и токен),  дата выставления офера  , кто сделал офер (адрес) , еспирайшн если указан (дата конца действия),  примерная цена в $ по текущему курсу. Сортировка по цене в $ по текущему курсу.  

История бидов по аукциону. В ответе:  сумма ставки,  % выше предыдущей ставки,  примерная цена в $ по текущему курсу,  дата ставки, адрес того кто сделал ставку. Сортировка цене.

Список бидов по овнеру который он сделал. Фильтр: по коллекции /ям, перебит или активен.  В ответе:  название нфт, сумма ставки, токен,   % выше предыдущей ставки,  примерная цена в $ по текущему курсу,  дата ставки, адрес нфт, изображение нфт, перебит или активен, название коллекции и ее адрес. Сортировка по дате.

Список бидов по овнеру которые он получил на активные аукционы (последний бид по кажому аукциону). Фильтр: по коллекции /ям. В ответе:  название нфт,  сумма ставки,  % выше предыдущей ставки,  примерная цена в $ по текущему курсу,  дата ставки, адрес того кто сделал ставку адрес нфт, изображение нфт. Сортировка дате.

Список оферов по овнеру которые он сделал. Фильтры: коллекция/ям, активен или нет. В ответе: Название нфт, изображение нфт, адрес нфт, название коллекции, адрес коллекции, дата офера, время до конца действия, истек или активен, токен, цена, % от предыдущего офера,  цена в $ по текущему курсу. Сортирвока по цене в $

Список оферов по овнеру которые сделаны на его нфт. Фильтры: коллекция/ии, активные или нет. В ответе: Название нфт, изображение нфт, адрес нфт, название коллекции, адрес коллекции, дата офера, время до конца действия, истек или активен, токен, цена, % от предыдущего офера,  цена в $ по текущему курсу, то сделал офер(адрес). Сортирвока по цене в $

(не в первую очередь) Список евентов. Фильтры: по нфт, по коллекции, все, по овнеру (те которые овнер сделал и те ивенты которые относяться к его нфт), типу евента. В ответе: все информация которая содержится в евентах и его тип(бид, офер, трансфер и т.п.). Сортирвока по дате евента.
Общее количество элеиментов в ответе.
Пейджинг где это возможно. Пейдж сайз и номер страницы параметами.

Helen BF, [11.08.2022 00:41]
Примерный перечень ручек